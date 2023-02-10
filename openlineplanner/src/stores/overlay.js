import { defineStore } from "pinia";
import { useLinesStore } from "./lines";
import { randomColor } from "../helpers/random";

export const useOverlayStore = defineStore({
  id: "overlay",
  state: () => ({
    overlay: "none",
    overlayData: {},
    showNameTags: true,
    showDistanceTags: false,
    showSpeedTags: false,
    exporting: false,
    coverage: "none",
    coverageData: {},
    currentRequestController: null,
  }),
  actions: {
    selectOverlay(type) {
      this.overlay = type;
      this.overlayData = {};
      // Send api parameters to Matomo
      if (window && window.Piwik) {
        window.Piwik.getTracker().trackEvent("editing", "overlay_select", type);
      }
      if (type != "none") {
        fetch(import.meta.env.VITE_API_ENDPOINT + "/overlay/" + type , {
          method: "GET",
          headers: {
            "Content-type": "application/json",
          },
        })
          .then((data) => data.json())
          .then((overlayData) => {
            this.overlayData = overlayData;
          });
      }
    },
    async updateCoverage() {
      if (this.coverage != "none") {
        this.loadCoverage();
      }
    },
    selectCoverage(style) {
      this.coverage = style;
      if (Object.keys(this.coverageData).length <= 0 && style != "none") {
        this.loadCoverage();
        return;
      }
      if (style == "none") {
        this.coverageData = {};
      }
    },
    loadCoverage() {
      const linesStore = useLinesStore();
      const stations = 
        linesStore.getPoints
          .filter((point) => point.type === "station")
          .map((station) => ({
            location: {
              y: station.lat,
              x: station.lng,
            },
            id: station.id,
            coverage: Math.max(
              ...station.lines.map((lineId) =>
                linesStore.getLineById(lineId).getCoverage()
              )
            ),
          }));
      // cancel request if duplicate exists
      if (this.currentRequestController != null) {
        this.currentRequestController.abort();
      }
      this.currentRequestController = new AbortController();
      const abortSignal = this.currentRequestController.signal;
      // Send api parameters to Matomo
      if (window && window.Piwik) {
        window.Piwik.getTracker().trackEvent(
          "editing",
          "coverage-info",
          stations
        );
      }
      fetch(import.meta.env.VITE_API_ENDPOINT + "/coverage-info", {
        method: "POST",
        body: JSON.stringify(stations),
        headers: {
          "Content-type": "application/json",
        },
        signal: abortSignal,
      })
        .then((data) => data.json())
        .then((overlayData) => {
          const distanceColors = [
            "#47aee9",
            "#6dc490",
            "#75ec62",
            "#eec83f",
            "#ED983E",
            "#ba546c",
            "#501478",
            "#371D49",
          ];

          const stationColorMapping = {};
          Object.values(linesStore.points).forEach((point) => {
            if (point.type === "station") {
              stationColorMapping[point.id] = randomColor();
            }
          });

          overlayData.features.forEach((point) => {
            point.properties["station_color"] =
              stationColorMapping[point.properties["closest_station"]];
            point.properties["distance_color"] =
              distanceColors[Math.floor(point.properties.distance / 100)];
          });
          this.coverageData = overlayData;
          this.currentRequestController = null;
        });
    },
    toggleNameTags() {
      this.showNameTags = !this.showNameTags;
    },
    toggleDistanceTags() {
      this.showDistanceTags = !this.showDistanceTags;
    },
    toggleSpeedTags() {
      this.showSpeedTags = !this.showSpeedTags;
    },
  },
});
