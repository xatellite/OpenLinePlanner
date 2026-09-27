//! Shared HTTP client and retry policy for the external geodata services.

use std::{sync::LazyLock, time::Duration};

use reqwest::StatusCode;

/// Nominatim's usage policy requires a contactable identifier, and reqwest
/// sends no User-Agent by default.
pub const USER_AGENT: &str = concat!(
    "OpenLinePlanner/",
    env!("CARGO_PKG_VERSION"),
    " (+https://openlineplanner.com)"
);

/// reqwest applies no timeout of its own, so without this a stuck request pins
/// a worker indefinitely.
const REQUEST_TIMEOUT: Duration = Duration::from_secs(60);

const MAX_ATTEMPTS: u32 = 3;
const RETRY_BACKOFF: Duration = Duration::from_secs(1);

static CLIENT: LazyLock<reqwest::Client> = LazyLock::new(|| {
    reqwest::Client::builder()
        .user_agent(USER_AGENT)
        .timeout(REQUEST_TIMEOUT)
        .build()
        .expect("failed to build http client")
});

pub fn client() -> &'static reqwest::Client {
    &CLIENT
}

/// `None` means the request failed before a status arrived. Any 4xx other than
/// 429 is a malformed request on our side and would fail identically on retry.
fn status_is_retryable(status: Option<StatusCode>) -> bool {
    match status {
        Some(status) => status.is_server_error() || status == StatusCode::TOO_MANY_REQUESTS,
        None => true,
    }
}

fn backoff_for(attempt: u32) -> Duration {
    RETRY_BACKOFF * 2u32.pow(attempt.saturating_sub(1))
}

/// `build` is called afresh per attempt because sending consumes the builder.
pub async fn send_with_retry<F>(service: &str, build: F) -> Result<String, reqwest::Error>
where
    F: Fn() -> reqwest::RequestBuilder,
{
    let mut attempt = 1;

    loop {
        let outcome = match build()
            .send()
            .await
            .and_then(|response| response.error_for_status())
        {
            Ok(response) => response.text().await,
            Err(error) => Err(error),
        };

        let error = match outcome {
            Ok(body) => return Ok(body),
            Err(error) => error,
        };

        if attempt >= MAX_ATTEMPTS || !status_is_retryable(error.status()) {
            return Err(error);
        }

        let backoff = backoff_for(attempt);
        log::warn!(
            "{} attempt {}/{} failed ({}); retrying in {:?}",
            service,
            attempt,
            MAX_ATTEMPTS,
            error,
            backoff
        );
        actix_web::rt::time::sleep(backoff).await;
        attempt += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn retries_transient_upstream_failures() {
        assert!(status_is_retryable(Some(StatusCode::GATEWAY_TIMEOUT)));
        assert!(status_is_retryable(Some(StatusCode::BAD_GATEWAY)));
        assert!(status_is_retryable(Some(StatusCode::SERVICE_UNAVAILABLE)));
        assert!(status_is_retryable(Some(StatusCode::INTERNAL_SERVER_ERROR)));
        assert!(status_is_retryable(Some(StatusCode::TOO_MANY_REQUESTS)));
        assert!(status_is_retryable(None));
    }

    #[test]
    fn does_not_retry_our_own_bad_requests() {
        assert!(!status_is_retryable(Some(StatusCode::BAD_REQUEST)));
        assert!(!status_is_retryable(Some(StatusCode::NOT_ACCEPTABLE)));
        assert!(!status_is_retryable(Some(StatusCode::OK)));
    }

    #[test]
    fn backoff_doubles_per_attempt() {
        assert_eq!(backoff_for(1), Duration::from_secs(1));
        assert_eq!(backoff_for(2), Duration::from_secs(2));
        assert_eq!(backoff_for(0), Duration::from_secs(1));
    }
}
