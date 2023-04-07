import { defineStore } from "pinia";

export const useUIStore = defineStore({
  id: "ui",
  state: () => ({
    mapStyle: "mapbox://styles/mapbox/light-v9",
    mode: "light",
  }),
  actions: {
    toggleMode() {
      if (this.mode === "light") {
        this.mode = "dark";
        this.mapStyle = "mapbox://styles/mapbox/dark-v9";
      } else {
        this.mode = "light";
        this.mapStyle = "mapbox://styles/mapbox/light-v9";
      }
      document.documentElement.setAttribute('data-theme', this.mode);
    },
  },
});
