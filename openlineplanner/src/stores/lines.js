import { defineStore } from "pinia";
import { randomColor } from "@/helpers/random";
import TransportLine from "@/helpers/classes/TransportLine";

export const useLinesStore = defineStore({
  id: "lines",
  state: () => ({
    lines: [],
    lineIdCounter: 1,
  }),
  getters: {
    getLineById: (state) => {
      return (id) => state.lines.find((line) => line.id == id);
    },
  },
  actions: {
    addLine() {
      const lineId = this.lineIdCounter;
      this.lineIdCounter++;
      const newLine = new TransportLine(lineId, `Line ${lineId}`);
      newLine.color = randomColor();
      this.lines = [...this.lines, newLine];
      return newLine.id;
    },
  },
});
