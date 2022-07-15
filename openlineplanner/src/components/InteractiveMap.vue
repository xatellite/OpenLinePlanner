<template>
  <div id="map" />
</template>

<script>
import mapboxgl from "mapbox-gl";
import "mapbox-gl/dist/mapbox-gl.css";
import { useEditStore } from "@/stores/editing";
import { useLinesStore } from "@/stores/lines";
import MapLinePoint from "./MapLinePoint.vue";
import { createApp, nextTick, watch } from "vue";
import { calculateMidPoint } from "@/helpers/geometry";
import VueApexCharts from "vue3-apexcharts";
import { usePaxStore } from "../stores/pax";
import { useOverlayStore } from "../stores/overlay";

export default {
  setup() {
    const editStore = useEditStore();
    const linesStore = useLinesStore();
    const paxStore = usePaxStore();
    const overlayStore = useOverlayStore();

    return {
      editStore,
      linesStore,
      paxStore,
      overlayStore,
    };
  },
  data() {
    return {
      map: null,
      referenceMarkers: [],
      pointMarkers: {},
      lines: {},
    };
  },
  mounted() {
    mapboxgl.accessToken =
      "pk.eyJ1IjoidGhlbmV3Y2l2aWxpYW4iLCJhIjoiY2pncDJwYWl0MDBvdTMzbWxpcHBoc24wNCJ9.kUw_kFeEUVVqE6-1l4VsIw";
    this.map = new mapboxgl.Map({
      container: "map",
      style: "mapbox://styles/mapbox/light-v9",
      center: [16.4585, 48.2284],
      zoom: 12,
    });
    this.map.on("load", () => {
      this.loadState();
    });
    this.map.on("mousedown", (e) => {
      if (e.originalEvent.target == this.map.getCanvas()) {
        const { lng, lat } = e.lngLat;
        if (this.editStore.isExtending !== null) {
          this.linesStore.addPoint(
            lat,
            lng,
            this.editStore.isEditing,
            this.editStore.isExtending
          );
        }
      }
    });

    this.map.on("zoom", () => {
      this.linesStore.getLines.forEach((line) => {
        this.updateLine(line);
      });
    });

    // Subscribe both stores
    this.linesStore.$onAction(({ name, after, args }) => {
      if (name === "addPoint") {
        after((result) => {
          this.addPoint(result);
          const line = this.linesStore.getLineById(result.lines[0]);
          this.updateLine(line);
        });
      }
      if (name === "addLine") {
        after((result) => {
          this.addLine(result);
        });
      }
      if (name === "addPointToLine") {
        after((result) => {
          this.updateLine(result);
        });
      }
      if (name === "loadState") {
        after(() => {
          this.loadState();
        });
      }
      if (name === "removePoint") {
        const pointRef = args[0];
        this.pointMarkers[pointRef].vue.unmount();
        this.pointMarkers[pointRef].remove();
        after((linesToBeUpdated) => {
          linesToBeUpdated.forEach((lineRef) => {
            const line = this.linesStore.lines[lineRef];
            this.updateLine(line);
          });
        });
      }
      if (name === "removeLine") {
        const line = args[0];
        console.log(line);
        this.removeLine(line);
        after((pointsToBeRemoved) => {
          pointsToBeRemoved.forEach((pointRef) => {
            this.pointMarkers[pointRef].vue.unmount();
            this.pointMarkers[pointRef].remove();
          });
          this.drawReferencePoints();
        });
      }
      if (name === "updateLineColor") {
        const lineRef = args[0];
        const line = this.linesStore.lines[lineRef];
        after(() => {
          this.updateLineStyle(line);
        });
      }
      if (
        name === "checkForParallelLine" ||
        name === "checkParallelsStillExist"
      ) {
        after((linesUpdated) => {
          if (linesUpdated) {
            linesUpdated.forEach((lineRef) =>
              this.updateLine(this.linesStore.getLineById(lineRef))
            );
          }
        });
      }
    });

    watch(
      () => this.editStore.isEditing,
      () => {
        this.drawReferencePoints();
      }
    );

    watch(
      () => this.overlayStore.overlayData,
      () => {
        this.updateOverlay();
      }
    );
  },
  methods: {
    loadState() {
      Object.values(this.pointMarkers).forEach((pointMarker) => {
        pointMarker.vue.unmount();
        pointMarker.remove();
      });
      this.pointMarkers = {};
      this.linesStore.getLines.forEach((line) => {
        this.addLine(line);
      });
      this.linesStore.getPoints.forEach((point) => {
        this.addPoint(point);
      });
    },
    addLine(line) {
      this.map.addSource(line.getLineLongId(), {
        type: "geojson",
        data: this.linesStore.getLineString(line.id, this.map.getZoom()),
      });
      this.map.addLayer({
        id: line.getLineLongId(),
        type: "line",
        source: line.getLineLongId(),
        layout: {
          "line-join": "round",
          "line-cap": "round",
        },
        paint: {
          "line-color": line.color,
          "line-width": 5,
        },
      });
      this.lines[line.id] = line;
    },
    updateLine(line) {
      const geoJson = this.linesStore.getLineString(
        line.id,
        this.map.getZoom()
      );
      this.map.getSource(line.getLineLongId()).setData(geoJson);
      this.drawReferencePoints();
    },
    removeLine(line) {
      if (this.map.getSource(line.getLineLongId())) {
        this.map.removeLayer(line.getLineLongId());
        this.map.removeSource(line.getLineLongId());
      }
      delete this.lines[line.id];
    },
    updateLineStyle(line) {
      this.map.setPaintProperty(line.getLineLongId(), "line-color", line.color);
    },
    addPoint(point) {
      let domContainer;
      if (point.refMarker && point.refMarker !== null) {
        domContainer = point.refMarker.getElement();
      } else {
        domContainer = document.createElement("div");
      }
      const mapPoint = createApp(MapLinePoint, { pointRef: point.id });
      mapPoint.use(this.store);
      mapPoint.use(VueApexCharts);

      let newMarker;
      if (point.refMarker && point.refMarker !== null) {
        newMarker = point.refMarker;
      } else {
        newMarker = new mapboxgl.Marker(domContainer, { draggable: true });
        newMarker.setLngLat({ lat: point.lat, lng: point.lng }).addTo(this.map);
      }
      // Reset reference Marker to avoid cyclic references.
      point.refMarker = null;
      newMarker.pointRef = point.id;
      newMarker.vue = mapPoint;
      // Change this to onDrag to get more direct feedback -> sideeffects!
      // ToDo: Improve handle quality
      newMarker.on("dragend", this.onDragEnd);
      nextTick(() => {
        mapPoint.mount(domContainer);
      });
      this.pointMarkers[point.id] = newMarker;
    },
    onDragEnd(e) {
      const marker = e.target;
      if (marker.isReference) {
        const { lat, lng } = marker.getLngLat();
        marker.isReference = false;
        marker.getElement().classList.remove("line-reference-point");
        marker.on("drag", () => {});
        this.referenceMarkers = this.referenceMarkers.filter(
          (markerInList) => marker.refIndex !== markerInList.refIndex
        );
        this.linesStore.addPoint(
          lat,
          lng,
          this.editStore.isEditing,
          marker.refIndex,
          marker
        );
        this.updateLine(this.editStore.isEditing);
        this.drawReferencePoints();
      } else {
        const point = this.linesStore.getPointById(marker.pointRef);
        if (
          !this.editStore.isEditing ||
          !point.lines.includes(this.editStore.isEditing.id)
        ) {
          this.editStore.isEditing = this.linesStore.lines[point.lines[0]];
        }
        point.updatePosition(marker.getLngLat());
        // Update all effected lines
        point.lines.forEach((lineRef) => {
          const line = this.linesStore.getLineById(lineRef);
          this.updateLine(line);
        });

        if (point.type === "station") {
          this.paxStore.setCurrent(false);
        }
      }

      this.drawReferencePoints();
    },

    addReferencePoint(pointOne, pointTwo, refIndex) {
      const lngLat = calculateMidPoint(pointOne, pointTwo);
      const domContainer = document.createElement("div");
      domContainer.className = "line-reference-point";
      const newMarker = new mapboxgl.Marker(domContainer, { draggable: true });
      newMarker.isReference = pointOne;
      newMarker.refIndex = refIndex;
      newMarker.setLngLat(lngLat).addTo(this.map);
      newMarker.on("dragend", this.onDragEnd);
      this.referenceMarkers.push(newMarker);
    },

    drawReferencePoints() {
      this.referenceMarkers.forEach((marker) => marker.remove());
      if (this.editStore.isEditing) {
        this.referenceMarkers = [];
        const line = this.editStore.isEditing;
        line.pointIds.forEach((pointRef, index) => {
          if (index != 0) {
            this.addReferencePoint(
              this.linesStore.getPointById(line.pointIds[index - 1]),
              this.linesStore.getPointById(pointRef),
              index
            );
          }
        });
      }
    },

    updateOverlay() {
      if (this.map.getSource("overlay")) {
        this.map.removeLayer("overlay");
        this.map.removeSource("overlay");
      }
      if (
        this.overlayStore.overlayData &&
        this.overlayStore.overlay != "none"
      ) {
        this.map.addSource("overlay", {
          type: "geojson",
          data: this.overlayStore.overlayData,
        });
        let weight = "pop";
        if (this.overlayStore.overlay == "jobs") {
          weight = "jobs";
        } else if (this.overlayStore.overlay == "schools") {
          weight = "kids";
        }

        const colors = {
          residence: [
            "rgba(68,44,49, 0.1)",
            "#7F4955",
            "#BD6275",
            "#E0B092",
            "#F5E5C6",
          ],
          jobs: [
            "rgba(51,70,82, 0.1)",
            "#4F819E",
            "#4A99C6",
            "#92D6E0",
            "#C6F5DE",
          ],
          schools: [
            "rgba(82,75,51, 0.1)",
            "#CC9766",
            "#DBA95E",
            "#D6E092",
            "#E2F5C6",
          ],
        }[this.overlayStore.overlay];

        this.map.addLayer({
          id: "overlay",
          type: "heatmap",
          source: "overlay",
          paint: {
            // increase weight as diameter breast height increases
            "heatmap-weight": {
              property: weight,
              type: "exponential",
              stops: [
                [1, 0],
                [62, 1],
              ],
            },
            "heatmap-color": [
              "interpolate",
              ["linear"],
              ["heatmap-density"],
              0,
              colors[0],
              0.2,
              colors[1],
              0.4,
              colors[2],
              0.6,
              colors[3],
              0.8,
              colors[4],
            ],
          },
        });
      }
    },
  },
};
</script>

<style lang="scss">
@import "@/assets/variables.scss";
#map {
  position: relative;
  height: calc(100vh);
}

.line-reference-point {
  border: 12px solid transparent;
  border-radius: 100%;

  &::before {
    display: block;
    content: "";
    height: 8px;
    width: 8px;
    border-radius: 100%;
    background-color: rgba($c-box, 0.5);
  }
}
</style>
