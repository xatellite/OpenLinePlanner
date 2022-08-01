import { defineStore } from "pinia";

export const useOverlayStore = defineStore({
  id: "overlay",
  state: () => ({
    overlay: "none",
    overlayData: {},
    showNameTags: true,
    showDistanceTags: false,
    exporting: false,
  }),
  actions: {
    selectOverlay(type) {
      this.overlay = type;
      this.overlayData = {};
      if (type != "none") {
        fetch(import.meta.env.VITE_API_ENDPOINT + "/overlay", {
          method: "POST",
          body: JSON.stringify({ layer_name: type }),
          headers: {
            "Content-type": "application/json",
          },
        })
          .then((data) => data.json())
          .then((overlayData) => {
            this.overlayData = JSON.parse(overlayData.layerGeoJson);
          });
      }
    },
    toggleNameTags() {
      this.showNameTags = !this.showNameTags;
    },
    toggleDistanceTags() {
      this.showDistanceTags = !this.showDistanceTags;
    },
  },
});
