<template>
  <div id="map" />
</template>

<script>
import mapboxgl from "mapbox-gl";
import "mapbox-gl/dist/mapbox-gl.css";
import { useEditStore } from "@/stores/editing";
import { useLinesStore } from "@/stores/lines";
import MapLinePoint from "./MapLinePoint.vue";
import { createApp, nextTick } from "vue";
import { calculateMidPoint } from "@/helpers/geometry";

export default {
  setup() {
    const editStore = useEditStore();
    const linesStore = useLinesStore();

    return {
      editStore,
      linesStore,
    };
  },
  data() {
    return {
      map: null,
      referenceMarkers: [],
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
      // TODO: Here we want to load a layer
      // TODO: Here we want to load/setup the popup
    });
    // map.on('mousemove', (e) => {
    //   console.log()
    // });
    this.map.on("mousedown", (e) => {
      if (e.originalEvent.target == this.map.getCanvas()) {
        this.addLinePoint(e);
      }
    });

    window.addEventListener("keydown", (e) => {
      if (e.key === "Enter") {
        this.editStore.isExtending = null;
      }
    });

    // ToDo: Add outline shape
  },
  methods: {
    addLinePoint(e) {
      const { lng, lat } = e.lngLat;
      let pointRef;
      let lineRef;
      let currentLine;
      if (this.editStore.isAdding) {
        lineRef = this.editStore.isAdding;
        currentLine = this.linesStore.getLineById(lineRef);
        pointRef = currentLine.addPoint(lat, lng);
        this.editStore.isAdding = null;
        this.editStore.isExtending = lineRef;

        this.map.addSource(currentLine.getLineLongId(), {
          type: "geojson",
          data: currentLine.toLineString(),
        });
        this.map.addLayer({
          id: currentLine.getLineLongId(),
          type: "line",
          source: currentLine.getLineLongId(),
          layout: {
            "line-join": "round",
            "line-cap": "round",
          },
          paint: {
            "line-color": currentLine.color,
            "line-width": 5,
          },
        });
      } else if (this.editStore.isExtending) {
        lineRef = this.editStore.isExtending;
        currentLine = this.linesStore.getLineById(lineRef);
        pointRef = currentLine.addPoint(lat, lng);
        this.map
          .getSource(currentLine.getLineLongId())
          .setData(currentLine.toLineString());
        // ToDo: Add Reference Point
        this.drawReferencePoints();
      } else {
        return;
      }

      const currentPoint = currentLine.getPointById(pointRef);
      const domContainer = document.createElement("div");
      const mapPoint = createApp(MapLinePoint, { point: currentPoint });
      mapPoint.use(this.store);

      const newMarker = new mapboxgl.Marker(domContainer, { draggable: true });
      newMarker.pointRef = pointRef;
      newMarker.lineRef = lineRef;
      newMarker.setLngLat(e.lngLat).addTo(this.map);
      newMarker.on("drag", this.onDragEnd);

      nextTick(() => {
        mapPoint.mount(domContainer);
      });
    },
    onDragEnd(e) {
      const marker = e.target;
      console.log(e);
      let line;
      if (marker.isReference) {
        console.log("reference pressed");
        marker._element.className = marker._element.className.split(" ").filter((name) => name != "line-reference-point").join(" ");
        console.log(marker.lineReference);
        line = this.linesStore.getLineById(marker.lineReference);
        const { lat, lng } = marker.getLngLat();
        const pointRef = line.addPoint(lat, lng, marker.refIndex);
        const currentPoint = line.getPointById(pointRef);
        marker.pointRef = pointRef;
        console.log("ref", marker.lineReference);
        marker.lineRef = marker.lineReference;
        marker.isReference = false;
        const mapPoint = createApp(MapLinePoint, { point: currentPoint });
        mapPoint.use(this.store);
        nextTick(() => {
          console.log(mapPoint);
          mapPoint.mount(marker._element);
          console.log("mount");
        });
        this.referenceMarkers = this.referenceMarkers.filter(
          (listElement) => listElement.pointRef != marker.pointRef
        );
        console.log([...this.referenceMarkers]);
      } else {
        console.log(marker.lineRef);
        line = this.linesStore.getLineById(marker.lineRef);
        console.log(line);
        console.log(marker.pointRef);
        line.getPointById(marker.pointRef).updatePosition(marker.getLngLat());
      }
      this.map.getSource(line.getLineLongId()).setData(line.toLineString());

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
      if (this.editStore.isEditing) {
        this.referenceMarkers.forEach((marker) => marker.remove());
        this.referenceMarkers = [];
        const line = this.linesStore.getLineById(this.editStore.isEditing);
        line.points.forEach((point, index) => {
          if (index != 0) {
            this.addReferencePoint(line.points[index - 1], point, index);
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
    height: 5px;
    width: 5px; 
    border-radius: 100%;
    border: 2px solid $c-text-primary;
  }
}
</style>
