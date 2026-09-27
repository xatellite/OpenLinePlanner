import { defineStore } from "pinia";
import {
  getAdminBounds,
  getImportedRegions,
  getImportJob,
  postDataImport,
  postResolveExtract,
  searchPlace,
} from "../helpers/api";

const POLL_INTERVAL = 2000;
const TERMINAL_PHASES = ["done", "failed"];

// Deliberately a separate store from `data.js`: that one re-creates itself and
// re-fires two requests on every `useDataStore()` call, so extending it would
// multiply traffic for every new consumer.
export const useDataImportStore = defineStore({
  id: "dataImport",
  state: () => ({
    // "location" -> "area" -> "confirm" -> "progress"
    step: "location",
    searchQuery: "",
    searchResults: [],
    searching: false,
    point: null,
    areas: [],
    loadingAreas: false,
    selectedArea: null,
    resolved: null,
    resolving: false,
    importedRegions: [],
    job: null,
    error: null,
    pollHandle: null,
  }),
  actions: {
    async loadImportedRegions() {
      try {
        this.importedRegions = await getImportedRegions();
      } catch (error) {
        // Not fatal: the list is only used to mark areas as already available.
        console.error("failed to load imported regions", error);
      }
    },

    async search() {
      if (!this.searchQuery.trim()) return;
      this.searching = true;
      this.error = null;
      try {
        this.searchResults = await searchPlace(this.searchQuery);
      } catch (error) {
        this.error = "Place search failed. Check your connection and try again.";
      }
      this.searching = false;
    },

    async selectPoint(point) {
      this.point = point;
      this.areas = [];
      this.selectedArea = null;
      this.loadingAreas = true;
      this.error = null;
      this.step = "area";
      try {
        const response = await getAdminBounds(point);
        this.areas = response.features ?? [];
        if (this.areas.length === 0) {
          this.error =
            "No administrative areas found here. Try a point closer to a town or district centre.";
        }
      } catch (error) {
        this.error =
          "Could not look up areas for this location. The lookup service may be busy - please retry.";
      }
      this.loadingAreas = false;
    },

    async selectArea(area) {
      this.selectedArea = area;
      this.resolved = null;
      this.resolving = true;
      this.error = null;
      this.step = "confirm";
      try {
        this.resolved = await postResolveExtract(area, null);
      } catch (error) {
        this.error = String(error.message || error);
      }
      this.resolving = false;
    },

    async startImport() {
      if (!this.selectedArea) return;
      this.error = null;
      try {
        const accepted = await postDataImport(this.selectedArea, null);
        this.job = accepted.job;
        this.step = "progress";
        if (accepted.already_available) {
          await this.loadImportedRegions();
          return;
        }
        this.startPolling();
      } catch (error) {
        this.error = String(error.message || error);
      }
    },

    startPolling() {
      this.stopPolling();
      this.pollHandle = setInterval(async () => {
        if (!this.job) return this.stopPolling();
        try {
          this.job = await getImportJob(this.job.id);
        } catch (error) {
          console.error("failed to poll import job", error);
          return;
        }
        if (TERMINAL_PHASES.includes(this.job.phase)) {
          this.stopPolling();
          if (this.job.phase === "done") await this.loadImportedRegions();
        }
      }, POLL_INTERVAL);
    },

    stopPolling() {
      if (this.pollHandle) {
        clearInterval(this.pollHandle);
        this.pollHandle = null;
      }
    },

    back() {
      if (this.step === "confirm") {
        this.step = "area";
        this.selectedArea = null;
        this.resolved = null;
      } else if (this.step === "area") {
        this.step = "location";
        this.areas = [];
        this.point = null;
      }
      this.error = null;
    },

    reset() {
      this.stopPolling();
      this.step = "location";
      this.searchQuery = "";
      this.searchResults = [];
      this.point = null;
      this.areas = [];
      this.selectedArea = null;
      this.resolved = null;
      this.job = null;
      this.error = null;
    },
  },
});

// Overpass ids arrive as either "relation/1234" or a plain number, matching the
// two shapes the backend's AdminArea conversion accepts.
export const areaIdOf = (area) => {
  if (area == null) return null;
  const id = area.id;
  if (typeof id === "string") return Number(id.split("/").pop());
  return Number(id);
};
