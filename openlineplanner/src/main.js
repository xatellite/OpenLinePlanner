import { createApp } from "vue";
import { createPinia } from "pinia";

import App from "./App.vue";
import router from "./router";

const app = createApp(App);

const piniaStore = createPinia();
app.config.globalProperties.store = piniaStore;

app.use(piniaStore);
app.use(router);

app.mount("#app");
