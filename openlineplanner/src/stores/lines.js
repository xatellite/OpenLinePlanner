import { defineStore } from "pinia";
import { randomColor } from "@/helpers/random";
import TransportLine from "@/helpers/classes/TransportLine";
import LinePoint from "@/helpers/classes/LinePoint";

export const useLinesStore = defineStore({
  id: "lines",
  state: () => ({
    lines: {},
    points: {},
    lineIdCounter: 1,
  }),
  getters: {
    getLineById: (state) => {
      return (lineRef) => state.lines[lineRef];
    },
    getPointById: (state) => {
      return (pointRef) => state.points[pointRef];
    },
    getLineString: (state) => {
      return (lineRef) => {
        const line = state.lines[lineRef];
        return {
          type: "FeatureCollection",
          features: [
            {
              type: "Feature",
              geometry: {
                type: "LineString",
                coordinates: line.pointIds.map((pointRef) => {
                  const point = state.points[pointRef];
                  return [point.lng, point.lat];
                }),
              },
            },
          ],
        };
      };
    },
  },
  actions: {
    addLine() {
      const lineId = this.lineIdCounter;
      this.lineIdCounter++;
      const newLine = new TransportLine(lineId, `Line ${lineId}`);
      newLine.color = randomColor();
      this.lines[lineId] = newLine;
      return newLine;
    },
    addPoint(lat, lng, line, index = -5) {
      const point = new LinePoint(lat, lng, line.id);
      this.lines[line.id].addPoint(point.id, index);
      this.points[point.id] = point;
      return point;
    },
    addPointToLine(pointRef, lineRef, index) {
      this.getPointById(pointRef).lines.push(lineRef);
      this.getLineById(lineRef).addPoint(pointRef, index);
    },
    removePoint(pointRef) {
      const linesToBeUpdated = this.points[pointRef].lines;
      linesToBeUpdated.forEach((lineRef) => {
        const line = this.lines[lineRef];
        line.pointIds = line.pointIds.filter((point) => point != pointRef);
      });
      delete this.points[pointRef];
      return linesToBeUpdated;
    },
    removeLine(line) {
      const pointsToBeUpdated = line.pointIds;
      const pointsRemoved = [];
      pointsToBeUpdated.forEach((pointRef) => {
        const point = this.points[pointRef];
        if (point.lines.includes(line.id)) {
          point.lines = point.lines.filter((lineRef) => lineRef != line.id);
        }
        if (point.lines.length === 0) {
          pointsRemoved.push(point.id);
          delete this.points[point.id];
        }
      });
      delete this.lines[line.id];
      return pointsRemoved;
    },
    loadState(state) {
      Object.values(state.lines).forEach((line) => {
        this.lines[line.id] = TransportLine.fromObject(line);
      });
      Object.values(state.points).map((point) => {
        this.points[point.id] = LinePoint.fromObject(point);
      });
      this.lineIdCounter = state.lineIdCounter;
    },
  },
});
