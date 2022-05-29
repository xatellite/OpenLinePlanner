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
import { usePaxStore } from '../stores/pax';

export default {
  setup() {
    const editStore = useEditStore();
    const linesStore = useLinesStore();
    const paxStore = usePaxStore();

    return {
      editStore,
      linesStore,
      paxStore,
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
        this.removeLine(line);
        after((pointsToBeRemoved) => {
          pointsToBeRemoved.forEach((pointRef) => {
            this.pointMarkers[pointRef].vue.unmount();
            this.pointMarkers[pointRef].remove();
          });
          this.drawReferencePoints();
        });
      }
    });

    watch(
      () => this.editStore.isEditing,
      () => {
        this.drawReferencePoints();
      }
    );
  },
  methods: {
    loadState() {
      Object.values(this.lines).forEach((line) => this.removeLine(line));
      Object.values(this.pointMarkers).forEach((pointMarker) => {
        pointMarker.vue.unmount();
        pointMarker.remove();
      });
      this.pointMarkers = {};
      Object.values(this.linesStore.lines).forEach((line) =>
        this.addLine(line)
      );
      Object.values(this.linesStore.points).forEach((point) =>
        this.addPoint(point)
      );
    },
    addLine(line) {
      this.map.addSource(line.getLineLongId(), {
        type: "geojson",
        data: this.linesStore.getLineString(line.id),
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
      this.map
        .getSource(line.getLineLongId())
        .setData(this.linesStore.getLineString(line.id));
      this.drawReferencePoints();
    },
    removeLine(line) {
      this.map.removeLayer(line.getLineLongId());
      this.map.removeSource(line.getLineLongId());
      delete this.lines[line.id];
    },
    addPoint(point) {
      let domContainer;
      if (point.refMarker !== null) {
        domContainer = point.refMarker.getElement();
      } else {
        domContainer = document.createElement("div");
      }
      const mapPoint = createApp(MapLinePoint, { pointRef: point.id });
      mapPoint.use(this.store);
      mapPoint.use(VueApexCharts);

      let newMarker;
      if (point.refMarker !== null) {
        newMarker = point.refMarker;
      } else {
        newMarker = new mapboxgl.Marker(domContainer, { draggable: true });
        newMarker.setLngLat({ lat: point.lat, lng: point.lng }).addTo(this.map);
      }
      newMarker.pointRef = point.id;
      newMarker.vue = mapPoint;
      newMarker.on("drag", this.onDragEnd);
      nextTick(() => {
        mapPoint.mount(domContainer);
      });
      this.pointMarkers[point.id] = newMarker;
    },

    onDragEnd(e) {
      const marker = e.target;
      if (marker.isReference) {
        const { lat, lng } = marker.getLngLat();
        console.log(this.referenceMarkers);
        console.log(marker.refIndex);
        marker.isReference = false;
        marker.on("drag", () => {});
        this.referenceMarkers = this.referenceMarkers.filter((markerInList) => marker.refIndex !== markerInList.refIndex);
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
          this.paxStore.isCurrent = false;
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
      newMarker.lineReference = this.editStore.isEditing;
      newMarker.refIndex = refIndex;
      newMarker.setLngLat(lngLat).addTo(this.map);
      newMarker.on("drag", this.onDragEnd);
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
  },
};
</script>

<style lang="scss">
@import "@/assets/variables.scss";
#map {
  height: calc(100vh - $space-md);
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
