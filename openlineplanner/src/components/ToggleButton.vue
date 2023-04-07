<template>
  <div
    :class="`${
      active ? 'toggle-button toggle-button--active' : 'toggle-button'
    }`"
    @click="handleClick"
    @mousemove="handleMouseMove"
    @mouseenter="hovering = true"
    @mouseleave="hovering = false"
  >
    <span class="toggle-button__icon">
      <slot name="icon" />
    </span>
    <span class="toggle-button__text">
      <slot name="text" />
    </span>
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
    active: Boolean,
    callback: Function,
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
    handleClick(e) {
      e.stopPropagation();
      this.callback();
    },
    handleMouseMove(e) {
      this.tooltipOffsetX = e.x;
      this.tooltipOffsetY = e.y;
    },
  },
};
</script>

<style lang="scss" scoped>
.toggle-button {
  @extend button;
  position: relative;
  display: flex;
  align-items: center;
  justify-content: space-between;
  height: 2.5rem;
  width: auto;
  padding: $space-ssm $space-ssm;
  margin: 0;

  &__icon {
    padding: $space-ssm;
    border-right: 1px solid #d6d6d6;
  }

  &__text {
    flex-grow: 1;
    text-align: center;
  }

  &--active {
    background-color: var(--c-primary-light);
    color: var(--c-text-inverted);

    &:hover {
      background-color: var(--c-primary-light);
    }

    &:active {
      background-color: var(--c-tex-primary);
    }
    .toggle-button__icon {
      border-right: 1px solid #696969;
    }
  }
}
</style>
