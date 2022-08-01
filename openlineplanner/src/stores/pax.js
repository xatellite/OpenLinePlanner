import { defineStore } from "pinia";

const loadStationData = async () => {
  
}

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
    async loadStationData(linesStore) {
      const stations = {
        stations: linesStore.getPoints
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
        return this.stationData.stationInfo;
      }
      await this.loadStationData(linesStore);
      return this.stationData.stationInfo;
    },
    async getPaxForStation(stationRef, linesStore) {
      if (this.isCurrent) {
        return this.stationData.stationInfo[stationRef];
      }
      await this.loadStationData(linesStore);
      return this.stationData.stationInfo[stationRef];
    },
  },
});
