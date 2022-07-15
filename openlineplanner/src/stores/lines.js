import { defineStore } from "pinia";
import { randomColor } from "@/helpers/random";
import TransportLine from "@/helpers/classes/TransportLine";
import LinePoint from "@/helpers/classes/LinePoint";

export const useLinesStore = defineStore({
  id: "lines",
  state: () => ({
    lines: {},
    parallels: [], // { points:, count: }
    points: {},
    lineIdCounter: 1,
  }),
  getters: {
    getLines: (state) => Object.values(state.lines),
    getPoints: (state) => Object.values(state.points),
    getLineById: (state) => {
      return (lineRef) => state.lines[lineRef];
    },
    getPointById: (state) => {
      return (pointRef) => state.points[pointRef];
    },
    getLineString(state) {
      return (lineRef, zoomLevel) => {
        const line = state.lines[lineRef];
        const coordinates = [];
        const placeParallelDivergent = (div, point) => {
          if (div) {
            const lineIndex =
              Math.floor(div.lines.length / 2) - div.lines.indexOf(lineRef);
            const latDif =
              this.points[div.points[0]].lat - this.points[div.points[1]].lat;
            const lngDif =
              this.points[div.points[0]].lng - this.points[div.points[1]].lng;
            const lineLength =
              Math.sqrt(latDif ** 2 + lngDif ** 2) * zoomLevel ** 3 * 10;
            // Latitude / Longitude switched to get orthogonal line
            coordinates.push([
              point.lng -
                (latDif / lineLength / (zoomLevel ** 3 / 20000)) * lineIndex, //((latDif / (zoomLevel**2)) * lineIndex),
              point.lat +
                (lngDif / lineLength / (zoomLevel ** 3 / 20000)) * lineIndex, //(lngDif / (zoomLevel**2)) * lineIndex,
            ]);
          }
        };
        line.pointIds.forEach((pointRef, index) => {
          const point = state.points[pointRef];
          // Add parallel line merge point
          if (index > 0) {
            const parallelExit = this.parallels.find(
              (parallel) =>
                parallel.points.includes(pointRef) &&
                parallel.points.includes(line.pointIds[index - 1])
            );
            placeParallelDivergent(parallelExit, point);
          }
          coordinates.push([point.lng, point.lat]);
          // Add parallel line spread point
          if (line.pointIds.length > index + 2) {
            const parallelEntry = this.parallels.find(
              (parallel) =>
                parallel.points.includes(pointRef) &&
                parallel.points.includes(line.pointIds[index + 1])
            );
            placeParallelDivergent(parallelEntry, point);
          }
        });
        return {
          type: "FeatureCollection",
          features: [
            {
              type: "Feature",
              geometry: {
                type: "LineString",
                coordinates,
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
      const newLine = new TransportLine(`Line ${lineId}`);
      newLine.color = randomColor();
      this.lines[newLine.id] = newLine;
      return newLine;
    },
    // domElement given for InteractiveMap to handle this action and remove Reference again.
    // ToDo: Find smarter way to do this.
    addPoint(lat, lng, line, index = -1, refMarker = null) {
      const point = new LinePoint(lat, lng, line.id);
      this.lines[line.id].addPoint(point.id, index);
      point.refMarker = refMarker;
      this.points[point.id] = point;
      this.checkParallelsStillExist();
      return point;
    },
    // Referencing a point to line without adding a new Point
    addPointToLine(pointRef, lineRef, index) {
      this.points[pointRef].lines.push(lineRef);
      this.lines[lineRef].addPoint(pointRef, index);
      this.checkForParallelLine(this.points[pointRef], this.lines[lineRef]);
      return this.lines[lineRef];
    },
    removePoint(pointRef) {
      const linesToBeUpdated = this.points[pointRef].lines;
      linesToBeUpdated.forEach((lineRef) => {
        const line = this.lines[lineRef];
        const pointIndex = line.pointIds.indexOf(pointRef);
        line.pointIds = line.pointIds.filter((point) => point != pointRef);
        if (line.pointIds.length >= pointIndex) {
          this.checkForParallelLine(
            this.points[line.pointIds[pointIndex]],
            line
          );
        }
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
      this.checkParallelsStillExist();
      return pointsRemoved;
    },
    updateLineColor(line, color) {
      this.lines[line].color = color;
    },
    loadState(savedState) {
      this.parallels = [];
      Object.keys(this.lines).forEach((lineRef) => this.removeLine(this.lines[lineRef]));
      Object.values(savedState.lines).forEach((line) => {
        this.lines[line.id] = TransportLine.fromObject(line);
      });
      Object.values(savedState.points).map((point) => {
        this.points[point.id] = LinePoint.fromObject(point);
      });

      this.lineIdCounter = savedState.lineIdCounter;
      this.parallels = savedState.parallels;
    },
    /*
     * Check if a parallel exists with the new merged point.
     * @return list of effected lines.
     */
    checkForParallelLine(lastPoint, currentLine) {
      if (lastPoint.lines.length > 1 && currentLine.pointIds.length > 1) {
        // Get previous point added.
        const previousPointRef =
          currentLine.pointIds[currentLine.pointIds.indexOf(lastPoint.id) - 1];
        // Check all Lines for parallels
        const parallelLineIds = lastPoint.lines
          .map((line) => {
            if (line == currentLine.id) return currentLine.id;
            const indexOfPrevious =
              this.lines[line].pointIds.indexOf(previousPointRef);
            const indexOfCurrent = this.lines[line].pointIds.indexOf(
              lastPoint.id
            );
            if (
              indexOfPrevious >= 0 &&
              Math.abs(indexOfCurrent - indexOfPrevious) === 1
            ) {
              return line;
            }
          })
          .filter((element) => element);
        if (parallelLineIds.length > 1) {
          parallelLineIds.sort((lineId) =>
            Object.keys(this.lines).indexOf(lineId)
          );
          this.parallels = [
            ...this.parallels.filter(
              (parallel) =>
                !(
                  parallel.points.includes(previousPointRef) &&
                  parallel.points.includes(lastPoint.id)
                )
            ),
            {
              points: [previousPointRef, lastPoint.id],
              lines: parallelLineIds,
            },
          ];
          return parallelLineIds;
        }
      }
      return [];
    },
    /*
     * Check if a parallel exists with the new merged point.
     * @return list of effected lines.
     */
    checkParallelsStillExist() {
      // Check if parallel is still part of the lines
      this.parallels.forEach((parallel) => {
        parallel.lines = parallel.lines.filter((lineRef) => {
          if (!this.lines[lineRef]) {
            return false;
          }
          const linePoints = this.lines[lineRef].pointIds;
          if (
            linePoints.indexOf(parallel.points[0]) &&
            linePoints.indexOf(parallel.points[1]) &&
            Math.abs(
              linePoints.indexOf(parallel.points[0]) -
                linePoints.indexOf(parallel.points[1])
            ) != 1
          ) {
            return false;
          }
          return true;
        });
      });
      let lineRemoved = [];
      // Remove parallels with only one line
      this.parallels = this.parallels.filter((parallel) => {
        if (parallel.lines.length > 1) {
          return true;
        }
        lineRemoved = [...lineRemoved, ...parallel.lines];
        return false;
      });
      return Array.from(new Set(lineRemoved));
    },
  },
});
