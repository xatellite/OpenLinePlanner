<template>
  <div
    @click="selectPoint"
    :class="
      editStore.pointSelected === point.id
        ? 'map-marker map-marker--selected'
        : 'map-marker'
    "
  >
    <div
      :class="point.type"
      :style="
        point.type == 'line'
          ? `border-color: ${linesStore.getLineById(point.lines[0])?.color}`
          : ''
      "
    />
    <MapAddStationPopup
      v-if="editStore.pointSelected === point.id && point.type != 'station'"
      :point="point"
    />
    <MapStationPopup
      v-if="editStore.pointSelected === point.id && point.type === 'station'"
      :point="point"
    />
  </div>
</template>

<script>
import MapAddStationPopup from "./MapAddStationPopup.vue";
import MapStationPopup from "./MapStationPopup.vue";
import { useEditStore } from "../stores/editing";
import { useLinesStore } from "../stores/lines";
import { watch } from "vue";

export default {
  data() {
    return {
      editStore: useEditStore(),
      linesStore: useLinesStore(),
      point: useLinesStore().getPointById(this.pointRef),
    };
  },
  mounted() {
    watch(
      () => this.linesStore.points[this.pointRef],
      () => {
        this.point = this.linesStore.getPointById(this.pointRef);
      }
    );
    window.addEventListener("click", this.handleClickOutside);
  },
  props: {
    pointRef: String,
  },
  components: { MapAddStationPopup, MapStationPopup },
  beforeUnmount() {
    window.removeEventListener("click", this.handleClickOutside);
  },
  methods: {
    handleClickOutside(e) {
      if (e.target.className === "station-popup__title") {
        return;
      }
      if (
        e.target != this.$el &&
        this.editStore.pointSelected === this.point.id
      ) {
        this.editStore.pointSelected = null;
        this.editStore.isMerging = null;
      }
    },
    selectPoint(e) {
      if (e.target.className === "station-popup__title") {
        return;
      }
      e.stopPropagation();
      if (this.editStore.isMerging && this.point.type === "station") {
        const oldStation = this.editStore.isMerging;
        // Check old station has no line in common
        if (
          oldStation.lines.find((lineRef) => this.point.lines.includes(lineRef))
        ) {
          console.log("Merge not possible - same line detected");
          return;
        }
        oldStation.lines.forEach((lineRef) => {
          const line = this.linesStore.getLineById(lineRef);
          // Find position of old station in line
          const indexInLine = line.pointIds.findIndex(
            (pointRef) => pointRef === oldStation.id
          );
          // Add current Station as replacement
          line.pointIds.splice(indexInLine, 0, this.point.id);
          // Add old line to new Station
          this.point.lines = [...this.point.lines, lineRef];
        });
        // Remove old station
        this.linesStore.removePoint(oldStation.id);
        this.editStore.isMerging = null;
        return;
      }
      this.editStore.pointSelected = this.point.id;
      this.editStore.isExtending = null;
    },
  },
};
</script>

<style lang="scss">
@import "@/assets/variables.scss";
.map-marker {
  border: 10px solid transparent;
  border-radius: 100%;
  position: relative;

  &--selected {
    box-shadow: $bs-md;
  }
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
  border: 5px solid;
}
</style>
