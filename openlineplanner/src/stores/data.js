import { defineStore } from "pinia";
import { getMethods } from "../helpers/api";

export const useDataStore = () => {
  const innerStore = defineStore({
    id: "data",
    state: () => ({
      methods: null,
      addPoint: null,
      selectedMethod: null,
      answers: [],
    }),
    actions: {
      setAddPoint(value) {
        this.addPoint = value;
      },
      removeAddPoint() {
        this.addPoint = null;
      },
      async loadMethods() {
        this.methods = await getMethods();
      },
    },
  });
  // Async Init
  const s = innerStore();
  s.loadMethods();
  return s;
};
