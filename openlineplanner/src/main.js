import { createApp } from "vue";
import { createPinia } from "pinia";

import App from "./App.vue";
import router from "./router";
import { initAnalytics } from "./helpers/analytics";

const app = createApp(App);

const piniaStore = createPinia();
app.config.globalProperties.store = piniaStore;

app.use(piniaStore);
app.use(router);

initAnalytics();

app.mount("#app");
