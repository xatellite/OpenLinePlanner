<template>
  <div
    :class="
      overlayStore.coverage != 'none'
        ? 'select-container select-container--active'
        : 'select-container'
    "
    @click="handleClick"
    @mousemove="handleMouseMove"
    @mouseenter="hovering = true"
    @mouseleave="hovering = false"
  >
    <span>
      <LoadingIcon
        class="loader"
        v-if="
          overlayStore.coverage != 'none' &&
          Object.keys(overlayStore.coverageData).length === 0
        "
      />
      <BusStopIcon v-else-if="overlayStore.coverage == 'station'" />
      <MapMarkerDistanceIcon v-else-if="overlayStore.coverage == 'distance'" />
      <BlurOffIcon v-else />
    </span>
    <select @change="handleSelect" :value="overlayStore.coverage">
      <option value="none">None</option>
      <option value="station">Stations</option>
      <option value="distance">Distance</option>
    </select>
    <div
      class="tooltip"
      :style="`left: ${tooltipOffsetX}px; top: ${tooltipOffsetY}px`"
      v-if="hovering"
    >
      Select coverage overlay
    </div>
  </div>
</template>

<script>
import BusStopIcon from "vue-material-design-icons/BusStop.vue";
import MapMarkerDistanceIcon from "vue-material-design-icons/MapMarkerDistance.vue";
import BlurOffIcon from "vue-material-design-icons/BlurOff.vue";
import LoadingIcon from "vue-material-design-icons/Loading.vue";
import { useOverlayStore } from "../stores/overlay";

export default {
  components: {
    BusStopIcon,
    MapMarkerDistanceIcon,
    BlurOffIcon,
    LoadingIcon,
  },
  data() {
    return {
      overlayStore: useOverlayStore(),
      hovering: false,
      tooltipOffsetX: 0,
      tooltipOffsetY: 0,
    };
  },
  methods: {
    handleSelect(e) {
      e.stopPropagation();
      const selection = e.target.value;
      this.overlayStore.selectCoverage(selection);
    },
    handleClick(e) {
      e.stopPropagation();
    },
    handleMouseMove(e) {
      this.tooltipOffsetX = e.x;
      this.tooltipOffsetY = e.y;
    },
  },
};
</script>
