<template>
  <div id="map" class="map">
    <DataSourceMapOverlay />
  </div>
</template>

<script>
import mapboxgl from "mapbox-gl";
import "mapbox-gl/dist/mapbox-gl.css";
import DataSourceMapOverlay from "./DataSourceMapOverlay.vue";
import { useDataStore } from '../stores/data';

export default {
    data() {
        return {
            map: null,
            selectPoint: null,
            dataStore: useDataStore(),
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
            preserveDrawingBuffer: true,
        });
        this.map.on("load", () => {
            // ToDo: Do something?!
        });
        // Handle mouse click
        this.map.on("mousedown", (e) => {
            if (e.originalEvent.target == this.map.getCanvas()) {
                const { lng, lat } = e.lngLat;
                if (e.originalEvent.button === 2) {
                    this.dataStore.setAddPoint({ lng, lat });
                }
            }
        });
    },
    components: { DataSourceMapOverlay }
};
</script>

<style lang="scss" scoped>
.map {
  position: relative;
  height: 100%;
  width: 100%;
  border: 1px solid $c-button-border;
  border-radius: $br-md;
  overflow: hidden;

  @media (max-width: 700px), (max-height: 600px) {
    min-height: 550px;
  }
}
.mapboxgl-marker {
  z-index: 3;
}
</style>
