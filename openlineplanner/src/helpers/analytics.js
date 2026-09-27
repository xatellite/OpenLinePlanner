import { init, track } from "@plausible-analytics/tracker";

const DEFAULT_API_HOST = "https://analytics.xatellite.space";

// Bundled rather than loaded from a <script> tag: an analytics script that
// hangs keeps the document in `readyState: "interactive"`, and Chrome will not
// start SVG SMIL timelines until the load event fires, which froze every
// loading spinner in the app. Bundled code plus `fetch` cannot hold the load
// event.
let started = false;

export function initAnalytics() {
  if (started) return;

  const apiHost = (
    import.meta.env.VITE_PLAUSIBLE_URL || DEFAULT_API_HOST
  ).replace(/\/+$/, "");

  // Defaults to the serving host so one build works on every domain.
  const domain =
    import.meta.env.VITE_PLAUSIBLE_DOMAIN || window.location.hostname;

  try {
    init({
      domain,
      endpoint: `${apiHost}/api/event`,
      // The tracker hooks pushState/popstate itself, so vue-router navigation
      // needs no wiring.
      autoCapturePageviews: true,
      outboundLinks: true,
      captureOnLocalhost: false,
    });
    started = true;
  } catch (error) {
    console.warn("analytics unavailable", error);
  }
}

/**
 * `props` values must be scalars. Send counts, not payloads -- the planning
 * data holds user-drawn coordinates.
 */
export function trackEvent(name, props) {
  if (!started) return;
  try {
    track(name, props ? { props } : undefined);
  } catch (error) {
    console.warn("failed to record event", name, error);
  }
}
