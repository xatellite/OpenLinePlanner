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
    <div
      class="line-point__name-label"
      v-if="
        point.name &&
        !(editStore.pointSelected === point.id) &&
        overlayStore.showNameTags
      "
    >
      <span>{{ point.name }}</span>
    </div>
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
import { useOverlayStore } from "../stores/overlay";
import { watch } from "vue";

export default {
  data() {
    return {
      editStore: useEditStore(),
      linesStore: useLinesStore(),
      overlayStore: useOverlayStore(),
      point: useLinesStore().getPointById(this.pointRef),
    };
  },
  mounted() {
    watch(
      () => this.linesStore.points[this.pointRef],
      () => {
        if (this.linesStore.points[this.pointRef]) {
          this.$el.parentElement.classList.remove(`type-${this.point.type}`);
          this.point = this.linesStore.getPointById(this.pointRef);
          this.$el.parentElement.classList.add(`type-${this.point.type}`);
        }
      }
    );
    watch(
      () => this.editStore.pointSelected,
      (pointSelected) => {
        if (this.$el.parentElement.classList.contains("dialog-active")) {
          this.$el.parentElement.classList.remove("dialog-active");
        }
        if (pointSelected == this.point.id) {
          this.$el.parentElement.classList.add("dialog-active");
        }
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
        oldStation.lines.forEach((lineRef) => {
          this.linesStore.checkForParallelLine(
            this.point,
            this.linesStore.getLineById(lineRef)
          );
        });
        this.editStore.isMerging = null;
        return;
        // Handle merge during adding.
      } else if (
        this.editStore.isEditing &&
        !this.point.lines.includes(this.editStore.isEditing.id) &&
        this.editStore.isExtending &&
        this.point.type === "station"
      ) {
        this.linesStore.addPointToLine(
          this.point.id,
          this.editStore.isEditing.id,
          -1
        );
        return;
      }
      this.editStore.pointSelected = this.point.id;
      this.editStore.isExtending = null;
    },
  },
};
</script>

<style lang="scss" scoped>
.map-marker {
  border: 10px solid transparent;
  border-radius: 100%;
  position: relative;

  &--selected {
    box-shadow: $bs-md;
  }
}

.line-point {
  &__name-label {
    position: absolute;
    bottom: 110%;
    width: fit-content;
    padding: 1px $space-ssm;
    font-weight: 700;
    white-space: nowrap;
    background-color: $c-box;
    border: 2px solid $c-text-primary;
    border-radius: $br-md;
    z-index: 3;
  }
}

.line {
  border-radius: 100%;
  border: 5px solid;
}
</style>
