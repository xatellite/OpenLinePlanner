import { defineStore } from "pinia";

export const usePaxStore = defineStore({
  id: "pax",
  state: () => ({
    stationData: [],
    isCurrent: false,
    currentRequestController: null,
  }),
  actions: {
    setCurrent(isCurrent) {
      this.isCurrent = isCurrent;
      return isCurrent;
    },
    async getPaxForStation(stationRef, linesStore) {
      if (this.isCurrent) {
        return this.stationData.stationInfo.find(
          (station) => station.id === stationRef
        );
      }
      const stations = {
        stations: linesStore
          .getPoints
          .filter((point) => point.type === "station")
          .map((station) => ({
            lat: station.lat,
            lng: station.lng,
            id: station.id,
          })),
      };
      if (this.currentRequestController != null) {
        this.currentRequestController.abort();
      }

      this.currentRequestController = new AbortController();
      const abortSignal = this.currentRequestController.signal;
      const response = await fetch(
        "https://api.openlineplanner.xatellite.io/station-info",
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
      return this.stationData.stationInfo.find(
        (station) => station.id === stationRef
      );
    },
  },
});
