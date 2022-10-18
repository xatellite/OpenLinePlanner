import { createRouter, createWebHistory } from "vue-router";
import HomeView from "../views/HomeView.vue";
import LineInfo from "../views/LineInfo.vue";

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: "/",
      name: "home",
      component: HomeView,
    },
    {
      path: "/analysis",
      name: "analysis",
      component: LineInfo,
    },
  ],
});

export default router;
