import { defineStore } from "pinia";
import { useOverlayStore } from "./overlay";

export const usePaxStore = defineStore({
  id: "pax",
  state: () => ({
    stationData: [],
    isCurrent: false,
    calculationMethod: "absolute",
    routingMethod: "osm",
    currentRequestController: null,
  }),
  actions: {
    setCurrent(isCurrent) {
      const overlayStore = useOverlayStore();
      overlayStore.updateCoverage();
      this.isCurrent = isCurrent;
      return isCurrent;
    },
    toggleCalculationMethod() {
      this.isCurrent = false;
      if (this.calculationMethod === "absolute") {
        this.calculationMethod = "relative";
        return;
      }
      this.calculationMethod = "absolute";
    },
    toggleRoutingMethod() {
      if (this.routingMethod === "osm") {
        this.routingMethod = "naive";
        return;
      }
      this.routingMethod = "osm";
      this.setCurrent(false);
    },
    async loadStationData(linesStore) {
      const stations = {
        stations: linesStore.getPoints
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
          })),
        method: this.calculationMethod,
        routing: this.routingMethod
      };
      if (this.currentRequestController != null) {
        this.currentRequestController.abort();
      }

      this.currentRequestController = new AbortController();
      const abortSignal = this.currentRequestController.signal;
      // Send api parameters to Matomo
      if (window && window.Piwik) {
        window.Piwik.getTracker().trackEvent("editing", "station-info", stations);
      }
      const response = await fetch(
        import.meta.env.VITE_API_ENDPOINT + "/station-info",
        {
          method: "POST",
          body: JSON.stringify(stations),
          headers: {
            "Content-type": "application/json",
          },
          signal: abortSignal,
        }
      );
      if (response.ok) {
        this.stationData = await response.json();
        this.setCurrent(true);
      }
      this.currentRequestController = null;
    },
    async getPaxForAllStations(linesStore) {
      if (this.isCurrent) {
        return this.stationData;
      }
      await this.loadStationData(linesStore);
      return this.stationData;
    },
    async getPaxForStation(stationRef, linesStore) {
      if (this.isCurrent) {
        return this.stationData[stationRef];
      }
      await this.loadStationData(linesStore);
      return this.stationData[stationRef];
    },
  },
});
