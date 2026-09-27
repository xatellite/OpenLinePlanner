//! CORS policy. The frontend is served from a different host in every
//! deployment, so every browser request is cross-origin.

use std::sync::LazyLock;

use actix_cors::Cors;
use actix_web::http::header;

/// Origins allowed when `CORS_ALLOWED_ORIGINS` is unset.
const DEFAULT_ALLOWED_ORIGINS: &[&str] = &[
    "https://openlineplanner.com",
    "https://test.openlineplanner.com",
];

const ALLOWED_ORIGINS_ENV: &str = "CORS_ALLOWED_ORIGINS";

/// Falls back to the defaults when unset or blank, so an empty
/// `CORS_ALLOWED_ORIGINS=` does not lock every browser out.
fn resolve_allowed_origins(configured: Option<String>) -> Vec<String> {
    let origins: Vec<String> = configured
        .unwrap_or_default()
        .split(',')
        .map(|origin| origin.trim().trim_end_matches('/'))
        .filter(|origin| !origin.is_empty())
        .map(|origin| origin.to_owned())
        .collect();

    if origins.is_empty() {
        return DEFAULT_ALLOWED_ORIGINS
            .iter()
            .map(|origin| (*origin).to_owned())
            .collect();
    }

    origins
}

/// Loopback on any port -- dev servers pick their own. Parsed strictly rather
/// than prefix-matched, so `http://localhost.evil.com` does not pass.
fn is_loopback_origin(origin: &str) -> bool {
    let Some(rest) = origin
        .strip_prefix("http://")
        .or_else(|| origin.strip_prefix("https://"))
    else {
        return false;
    };

    if rest.contains('@') || rest.contains('/') {
        return false;
    }

    // Keep IPv6 literals intact when splitting off the port.
    let (host, port) = if rest.starts_with('[') {
        match rest.find(']') {
            Some(end) => (&rest[..=end], &rest[end + 1..]),
            None => return false,
        }
    } else {
        match rest.find(':') {
            Some(index) => (&rest[..index], &rest[index..]),
            None => (rest, ""),
        }
    };

    let port_is_valid = port.is_empty()
        || port
            .strip_prefix(':')
            .is_some_and(|port| !port.is_empty() && port.chars().all(|c| c.is_ascii_digit()));

    port_is_valid && matches!(host, "localhost" | "127.0.0.1" | "[::1]")
}

/// Resolved once: `HttpServer::new` runs its factory per worker thread.
static ALLOWED_ORIGINS: LazyLock<Vec<String>> = LazyLock::new(|| {
    let origins = resolve_allowed_origins(std::env::var(ALLOWED_ORIGINS_ENV).ok());
    log::info!("allowing cross-origin requests from {:?}", origins);
    origins
});

/// `Cors::default()` denies request headers too: without `allowed_headers`,
/// the preflight for any `Content-Type: application/json` POST gets a 400 and
/// every write endpoint is unreachable from a browser.
///
/// Uses `cfg!` rather than `#[cfg]` so both paths stay compiled.
pub fn build() -> Cors {
    if cfg!(debug_assertions) {
        return Cors::permissive();
    }

    let mut cors = Cors::default()
        .allowed_methods(vec!["GET", "POST", "DELETE", "PUT"])
        .allowed_headers(vec![header::CONTENT_TYPE, header::ACCEPT])
        // Alongside the allowlist, not instead of it. The API is
        // unauthenticated, so loopback gains a caller nothing curl would not.
        .allowed_origin_fn(|origin, _req| {
            origin.to_str().map(is_loopback_origin).unwrap_or(false)
        })
        .max_age(3600);

    for origin in ALLOWED_ORIGINS.iter() {
        cors = cors.allowed_origin(origin);
    }

    cors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn falls_back_to_defaults_when_unset_or_blank() {
        assert_eq!(resolve_allowed_origins(None), DEFAULT_ALLOWED_ORIGINS);
        assert_eq!(
            resolve_allowed_origins(Some("".into())),
            DEFAULT_ALLOWED_ORIGINS
        );
        assert_eq!(
            resolve_allowed_origins(Some("  ,  , ".into())),
            DEFAULT_ALLOWED_ORIGINS
        );
    }

    #[test]
    fn parses_a_comma_separated_list() {
        assert_eq!(
            resolve_allowed_origins(Some(
                "https://a.example.com,https://b.example.com".into()
            )),
            vec!["https://a.example.com", "https://b.example.com"]
        );
    }

    #[test]
    fn accepts_loopback_on_any_port() {
        assert!(is_loopback_origin("http://localhost:5173"));
        assert!(is_loopback_origin("http://localhost:5050"));
        assert!(is_loopback_origin("http://localhost"));
        assert!(is_loopback_origin("http://127.0.0.1:8080"));
        assert!(is_loopback_origin("http://[::1]:5173"));
        assert!(is_loopback_origin("http://[::1]"));
        assert!(is_loopback_origin("https://localhost:5173"));
    }

    #[test]
    fn rejects_hosts_that_merely_look_like_loopback() {
        assert!(!is_loopback_origin("http://localhost.evil.com"));
        assert!(!is_loopback_origin("http://evil.com@localhost"));
        assert!(!is_loopback_origin("http://notlocalhost"));
        assert!(!is_loopback_origin("http://127.0.0.1.evil.com"));
        assert!(!is_loopback_origin("http://localhost:5173/../evil"));
        assert!(!is_loopback_origin("http://localhost:notaport"));
        assert!(!is_loopback_origin("http://localhost:"));
        assert!(!is_loopback_origin("http://[::1"));
        assert!(!is_loopback_origin("file://localhost"));
        assert!(!is_loopback_origin("localhost:5173"));
        assert!(!is_loopback_origin("null"));
    }

    #[test]
    fn tolerates_spacing_and_trailing_slashes() {
        // An Origin never has a trailing slash, so one here would never match.
        assert_eq!(
            resolve_allowed_origins(Some(
                " https://a.example.com/ ,\nhttps://b.example.com , ".into()
            )),
            vec!["https://a.example.com", "https://b.example.com"]
        );
    }
}
