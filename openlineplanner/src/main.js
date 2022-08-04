import { createApp } from "vue";
import { createPinia } from "pinia";

import App from "./App.vue";
import router from "./router";
import VueMatomo from "vue-matomo";

const app = createApp(App);

const piniaStore = createPinia();
app.config.globalProperties.store = piniaStore;

app.use(piniaStore);
app.use(router);

// Tracking for DEMO site
app.use(VueMatomo, {
  // Configure your matomo server and site by providing
  host: "https://matomo.raildeals.org/",
  siteId: 5,
  // Enables automatically registering pageviews on the router
  router: router,
  // Enables link tracking on regular links. Note that this won't
  // work for routing links (ie. internal Vue router links)
  enableLinkTracking: true,
  // Require consent before sending tracking information to matomo
  requireConsent: false,
  // Whether to track the initial page view
  trackInitialView: true,
  // Run Matomo without cookies
  disableCookies: true,
  preInitActions: [],
});

app.mount("#app");
