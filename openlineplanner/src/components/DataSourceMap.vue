<template>
  <div id="map" class="data-map">
    <DataSourceMapOverlay />
    <DataSourceMapLoading />
  </div>
</template>

<script>
import { watch } from "vue";
import mapboxgl from "mapbox-gl";
import "mapbox-gl/dist/mapbox-gl.css";
import DataSourceMapOverlay from "./DataSourceMapOverlay.vue";
import { useDataStore } from "../stores/data";
import DataSourceMapLoading from "./DataSourceMapLoading.vue";

export default {
  data() {
    return {
      map: null,
      selectPoint: null,
      dataStore: useDataStore(),
      marker: null,
      displayedBounds: [],
    };
  },
  components: { DataSourceMapOverlay, DataSourceMapLoading },
  mounted() {
    mapboxgl.accessToken =
      "pk.eyJ1IjoidGhlbmV3Y2l2aWxpYW4iLCJhIjoiY2pncDJwYWl0MDBvdTMzbWxpcHBoc24wNCJ9.kUw_kFeEUVVqE6-1l4VsIw";
    this.map = new mapboxgl.Map({
      container: "map",
      style: "mapbox://styles/mapbox/light-v9",
      center: [16.4585, 48.2284],
      zoom: 12,
      preserveDrawingBuffer: true,
    });
    // disable map rotation using right click + drag
    this.map.dragRotate.disable();
    // disable map rotation using touch rotation gesture
    this.map.touchZoomRotate.disableRotation();
    this.map.on("load", () => {
      // ToDo: Do something?!
    });
    // Handle mouse click
    this.map.on("mousedown", (e) => {
      if (e.originalEvent.target == this.map.getCanvas()) {
        const { lng, lat } = e.lngLat;
        this.dataStore.setAddPoint({ lng, lat });
        this.setMarker({ lng, lat });
      }
    });

    if (this.dataStore.addPoint) {
      this.setMarker(this.dataStore.addPoint);
    }

    watch(
      () => this.dataStore.mapBounds,
      () => {
        this.updateBounds();
      }
    );

    watch(
      () => this.dataStore.mapHighligh,
      () => {
        this.updateHighlight();
      }
    );
  },
  methods: {
    setMarker(point) {
      if (this.marker) {
        this.marker.remove();
      }
      const domContainer = document.createElement("div");
      domContainer.classList = "marker";
      const marker = new mapboxgl.Marker(domContainer, { draggable: true });
      marker.setLngLat({ lat: point.lat, lng: point.lng }).addTo(this.map);
      marker.on("dragend", this.onDragEnd);
      this.marker = marker;
    },
    onDragEnd() {},
    updateBounds() {
      const bounds = this.dataStore.mapBounds;

      this.removeAllBoundLayers();

      bounds.forEach((bound) => {
        const boundId = bound.properties.id;
        this.map.addSource(boundId, {
          type: "geojson",
          data: bound,
        });

        this.map.addLayer({
          id: boundId,
          type: "fill",
          source: boundId,
          paint: {
            "fill-color": "#1B998B",
            "fill-opacity": 0.2,
          },
        });
        this.map.addLayer({
          id: `stroke-${boundId}`,
          type: "line",
          source: boundId,
          paint: {
            "line-color": "#1B998B",
            "line-opacity": 0.3,
            "line-width": 2,
          },
        });
        this.displayedBounds.push(boundId);
      });
    },
    updateHighlight() {
      const highlight = this.dataStore.mapHighligh;
      this.displayedBounds.forEach((boundId) => {
        this.map.setPaintProperty(boundId, "fill-opacity", 0.2);
        this.map.setPaintProperty(`stroke-${boundId}`, "line-opacity", 0.2);
      });
      if (highlight) {
        const boundId = highlight.properties.id;
        this.map.setPaintProperty(boundId, "fill-opacity", 0.5);
        this.map.setPaintProperty(`stroke-${boundId}`, "line-opacity", 0.8);
      }
    },
    removeAllBoundLayers() {
      this.displayedBounds.forEach((boundId) => {
        this.map.removeLayer(boundId);
        this.map.removeLayer(`stroke-${boundId}`);
        this.map.removeSource(boundId);
      });
      this.displayedBounds = [];
    },
  },
};
</script>

<style lang="scss">
.data-map {
  position: relative;
  height: 100%;
  width: 100%;
  border: 1px solid $c-button-border;
  border-radius: $br-md;
  overflow: hidden;

  @media (max-width: 700px), (max-height: 600px) {
    min-height: 550px;
  }

  .marker {
    border: $space-ssm solid $c-box;
    border-radius: 50%;
    background-color: $c-accent-two;
    height: $space-sm;
    width: $space-sm;
  }
  .mapboxgl-marker {
    z-index: 3;
  }
}
</style>
