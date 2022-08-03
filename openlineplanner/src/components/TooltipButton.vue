<template>
  <div
    @mousemove="handleMouseMove"
    @mouseenter="hovering = true"
    @mouseleave="hovering = false"
    class="tooltip-button"
  >
    <button @click="handler">
      <slot />
    </button>
    <div
      class="tooltip"
      :style="`left: ${tooltipOffsetX}px; top: ${tooltipOffsetY}px`"
      v-if="hovering"
    >
      {{ toolTip }}
    </div>
  </div>
</template>

<script>
export default {
  props: {
    handler: Function,
    toolTip: String,
  },
  data() {
    return {
      hovering: false,
      tooltipOffsetX: 0,
      tooltipOffsetY: 0,
    };
  },
  methods: {
    handleMouseMove(e) {
      this.tooltipOffsetX = e.x;
      this.tooltipOffsetY = e.y;
    },
  },
};
</script>

<style lang="scss" scoped>
.tooltip-button {
  position: relative;
}
</style>
