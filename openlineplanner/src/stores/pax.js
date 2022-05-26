import { defineStore } from "pinia";

export const usePaxStore = defineStore({
  id: "pax",
  state: () => ({
    stationData: [],
    isCurrent: false,
  }),
  actions: {
    async getPaxForStation(stationRef, linesStore) {
      if (this.isCurrent) {
        return this.stationData.stationInfo.find((station) => station.id === stationRef);
      }
      const stations = {
        stations: Object.values(linesStore.points)
          .filter((point) => point.type === "station")
          .map((station) => ({
            lat: station.lat,
            lng: station.lng,
            id: station.id,
          })),
      };
      this.stationData = await (
        await fetch("http://127.0.0.1:8000/station-info", {
          method: "POST",
          body: JSON.stringify(stations),
          headers: {
            "Content-type": "application/json",
          },
        })
      ).json();
      this.isCurrent = true;
      return this.stationData.stationInfo.find((station) => station.id === stationRef);
    },
  },
});
