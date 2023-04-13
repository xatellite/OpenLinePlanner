<template>
  <div
    :class="
      overlayStore.overlay != 'none'
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
          overlayStore.overlay != 'none' &&
          Object.keys(overlayStore.overlayData).length === 0
        "
      />
      <HomeAccountIcon v-else-if="overlayStore.overlay == 'Residential'" />
      <!-- <SchoolOutlineIcon v-else-if="overlayStore.overlay == 'Shopping'" /> -->
      <BriefcaseAccountIcon v-else-if="overlayStore.overlay == 'Workplace'" />
      <BlurOffIcon v-else />
    </span>
    <select @change="handleSelect" :value="overlayStore.overlay">
      <option value="none">None</option>
      <option value="Residential">Residential</option>
      <option value="Workplace">Workplaces</option>
    </select>
    <div
      class="tooltip"
      :style="`left: ${tooltipOffsetX}px; top: ${tooltipOffsetY}px`"
      v-if="hovering"
    >
      Select data overlay
    </div>
  </div>
</template>

<script>
import HomeAccountIcon from "vue-material-design-icons/HomeAccount.vue";
// import SchoolOutlineIcon from "vue-material-design-icons/SchoolOutline.vue";
import BriefcaseAccountIcon from "vue-material-design-icons/BriefcaseAccount.vue";
import BlurOffIcon from "vue-material-design-icons/BlurOff.vue";
import LoadingIcon from "vue-material-design-icons/Loading.vue";
import { useOverlayStore } from "../stores/overlay";

export default {
  components: {
    HomeAccountIcon,
    // SchoolOutlineIcon,
    BriefcaseAccountIcon,
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
      this.overlayStore.selectOverlay(selection);
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

<style lang="scss" scoped></style>
