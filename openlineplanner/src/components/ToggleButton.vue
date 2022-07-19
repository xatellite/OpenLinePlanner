<template>
  <div
    :class="`${
      active ? 'toggle-button toggle-button--active' : 'toggle-button'
    }`"
    @click="handleClick"
  >
    <span class="toggle-button__icon">
      <slot name="icon" />
    </span>
    <span class="toggle-button__text">
      <slot name="text" />
    </span>
  </div>
</template>

<script>
export default {
  props: {
    active: Boolean,
    callback: Function,
  },
  methods: {
    handleClick(e) {
      e.stopPropagation();
      this.callback();
    },
  },
};
</script>

<style lang="scss" scoped>
@import "@/assets/variables.scss";

.toggle-button {
  @extend button;
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
    background-color: $c-text-primary;
    color: $c-text-inverted;

    &:hover {
      background-color: lighten($c-text-primary, 8);
    }

    &:active {
      background-color: lighten($c-text-primary, 16);
    }
    .toggle-button__icon {
        border-right: 1px solid #696969;
    }
  }
}
</style>
