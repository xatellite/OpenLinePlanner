import { defineStore } from "pinia";
import { getMapCenter } from "../helpers/api";

export const useUIStore = defineStore({
  id: "ui",
  state: () => ({
    mapStyle: "mapbox://styles/mapbox/light-v9",
    mapPosition: [16.4585, 48.2284],
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
      document.documentElement.setAttribute("data-theme", this.mode);
    },
    async getMapCenter() {
      console.log("INIT");
      const mapCenter = await getMapCenter();
      this.setMapCenter([mapCenter.x, mapCenter.y]);
    },
    setMapCenter(center) {
      this.mapPosition = center;
    },
  },
});
