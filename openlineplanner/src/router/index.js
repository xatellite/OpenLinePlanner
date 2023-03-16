import { createRouter, createWebHistory } from "vue-router";
import PlanningView from "@/views/PlanningView.vue";
import TimetableView from "@/views/TimetableView.vue";
import ProjectView from "@/views/ProjectView.vue";

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: "/",
      name: "planning",
      component: PlanningView,
    },
    {
      path: "/timetable",
      name: "timetable",
      component: TimetableView,
    },
    {
      path: "/project",
      name: "project",
      component: ProjectView,
    },
  ],
});

export default router;
