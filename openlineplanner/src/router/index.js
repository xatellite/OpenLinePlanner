import { createRouter, createWebHistory } from "vue-router";
import PlanningView from "@/views/PlanningView.vue";
import TimetableView from "@/views/TimetableView.vue";
import ProjectView from "@/views/ProjectView.vue";
import DataSourcesView from "@/views/DataSourcesView.vue";

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: "/",
      name: "Data",
      component: DataSourcesView,
    },
    {
      path: "/planning",
      name: "Planning",
      component: PlanningView,
    },
    {
      path: "/timetable",
      name: "Timetable",
      component: TimetableView,
    },
    {
      path: "/project",
      name: "Project",
      component: ProjectView,
    },
  ],
});

export default router;
