<template>
  <div
    @mousemove="handleMouseMove"
    @mouseenter="hovering = true"
    @mouseleave="hovering = false"
    :class="active ? 'text-button text-button--active' : 'text-button'"
  >
    <button class="text-button__button" v-if="href.length === 0" @click="handler">
      <slot />
    </button>
    <router-link v-if="href.length > 0" :to="href">
      <slot />
    </router-link>
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
    };
  }
};
</script>

<style lang="scss" scoped>
.text-button__button {
    background-color: none;
    outline: none;
    width: auto !important;
    padding-right: 12px;
}

.text-button {
  position: relative;
  background-color: none;
  padding-right: 14px;

  &--active {
    button {
      color: $c-text-inverted;

      &:hover {
        outline: 1px lighten($c-text-inverted, 8);
        color: lighten($c-text-inverted, 8);
      }

      &:active {
        color: lighten($c-text-inverted, 16);
      }
    }
  }
}
</style>
