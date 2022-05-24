<template>
  <div @click="selectPoint" class="map-marker">
    <div
      :class="point.type"
      :style="
        point.type == 'line' ? `background-color: ${point.lines[0].color}` : ''
      "
    />
    <MapAddStationPopup
      v-if="editStore.pointSelected === point.id && point.type != 'station'"
      :point="point"
    />
    <MapStationPopup
      v-if="editStore.pointSelected === point.id && point.type === 'station'"
    />
  </div>
</template>

<script>
import MapAddStationPopup from "./MapAddStationPopup.vue";
import MapStationPopup from "./MapStationPopup.vue";
import { useEditStore } from "../stores/editing";
export default {
  data() {
    return {
      editStore: useEditStore(),
    };
  },
  props: {
    point: Object,
  },
  components: { MapAddStationPopup, MapStationPopup },
  methods: {
    selectPoint() {
      this.editStore.pointSelected = this.point.id;
    },
  },
};
</script>

<style lang="scss">
@import "@/assets/variables.scss";
.map-marker {
  border: 10px solid transparent;
  border-radius: 100%;
  width: 15px;
  height: 15px;
}

.station {
  border-radius: 100%;
  border: 2px solid $c-text-primary;
  background-color: $c-box;
  width: 15px;
  height: 15px;
}

.line {
  border-radius: 100%;
  width: 10px;
  height: 10px;
}
</style>
