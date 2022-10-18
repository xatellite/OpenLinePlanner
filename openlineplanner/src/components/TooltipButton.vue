<template>
  <div
    @mousemove="handleMouseMove"
    @mouseenter="hovering = true"
    @mouseleave="hovering = false"
    :class="active ? 'tooltip-button tooltip-button--active' : 'tooltip-button'"
  >
    <button v-if="href.length === 0" @click="handler">
      <slot />
    </button>
    <router-link v-if="href.length > 0" class="button" :to="href">
      <slot />
    </router-link>
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
    href: {
      type: String,
      default: "",
    },
    active: {
      type: Boolean,
      default: false,
    },
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

  &--active {
    button {
      background-color: $c-text-primary;
      color: $c-text-inverted;

      &:hover {
        background-color: lighten($c-text-primary, 8);
      }

      &:active {
        background-color: lighten($c-text-primary, 16);
      }
    }
  }
}
</style>
