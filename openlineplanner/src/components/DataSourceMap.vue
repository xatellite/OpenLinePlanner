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
import { useUIStore } from "../stores/ui";

export default {
  data() {
    return {
      map: null,
      selectPoint: null,
      dataStore: useDataStore(),
      uiStore: useUIStore(),
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
      style: this.uiStore.mapStyle,
      center: this.uiStore.mapPosition,
      zoom: 8,
      preserveDrawingBuffer: true,
    });
    // disable map rotation using right click + drag
    this.map.dragRotate.disable();
    // disable map rotation using touch rotation gesture
    this.map.touchZoomRotate.disableRotation();
    this.map.on("load", () => {
      // ToDo: Do something?!
    });

    this.map.on("moveend", () => {
      this.uiStore.mapPosition = this.map.getCenter();
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

    this.uiStore.$onAction(({ name, after, value }) => {
      if (name === "setMapCenter") {
        after(() => {
          this.map.flyTo({
            center: this.uiStore.mapPosition,
            essential: true,
            zoom: 12,
          });
        });
      }
    });

    watch(
      () => this.uiStore.mode,
      () => {
        this.map.setStyle(this.uiStore.mapStyle);
      }
    );

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
        const boundId = bound.id;
        this.map.addSource(`${boundId}`, {
          type: "geojson",
          data: bound,
        });

        if (bound.properties.type == "area") {
          this.map.addLayer({
            id: `area-${boundId}`,
            type: "fill",
            source: `${boundId}`,
            paint: {
              "fill-color": bound.properties.color,
              "fill-opacity": 0.2,
            },
          });
          this.map.addLayer({
            id: `stroke-${boundId}`,
            type: "line",
            source: `${boundId}`,
            paint: {
              "line-color": bound.properties.color,
              "line-opacity": 0.3,
              "line-width": 2,
            },
          });
        } else if (bound.properties.type == "points") {
          this.map.addLayer({
            id: `points-${boundId}`,
            type: "circle",
            source: `${boundId}`,
            paint: {
              "circle-radius": 6,
              "circle-color": bound.properties.color,
            },
          });
        }
        this.displayedBounds.push(boundId);
      });
    },
    updateHighlight() {
      const highlight = this.dataStore.mapHighligh;
      this.displayedBounds.forEach((boundId) => {
        if (this.map.getLayer(`area-${boundId}`)) {
          this.map.setPaintProperty(`area-${boundId}`, "fill-opacity", 0.2);
          this.map.setPaintProperty(`stroke-${boundId}`, "line-opacity", 0.2);
        }
      });
      if (highlight) {
        const boundId = highlight.id;
        if (this.map.getLayer(`area-${boundId}`)) {
          this.map.setPaintProperty(`area-${boundId}`, "fill-opacity", 0.5);
          this.map.setPaintProperty(`stroke-${boundId}`, "line-opacity", 0.8);
        }
      }
    },
    removeAllBoundLayers() {
      this.displayedBounds.forEach((boundId) => {
        if (this.map.getLayer(`area-${boundId}`)) {
          this.map.removeLayer(`area-${boundId}`);
          this.map.removeLayer(`stroke-${boundId}`);
        }
        if (this.map.getLayer(`points-${boundId}`)) {
          this.map.removeLayer(`points-${boundId}`);
        }
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
  border: 1px solid var(--c-button-border);
  border-radius: $br-md;
  overflow: hidden;

  @media (max-width: 700px), (max-height: 600px) {
    min-height: 550px;
  }

  .marker {
    border: $space-ssm solid var(--c-box);
    border-radius: 50%;
    background-color: var(--c-accent-two);
    height: $space-sm;
    width: $space-sm;
  }
  .mapboxgl-marker {
    z-index: 3;
  }
}
</style>
