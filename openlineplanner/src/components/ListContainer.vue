<template>
  <div class="list-container">
    <h5 class="list-container__title">{{ title }}</h5>
    <slot></slot>
  </div>
</template>

<script>
export default {
  props: {
    title: String,
  },
};
</script>

<style lang="scss" scoped>
.list-container {
  border: 1px solid var(--c-button-border);
  border-radius: $br-md;
  background-color: var(--c-box-darkened);
  height: 100%;
  width: 100%;
  max-height: 100%;
  overflow: scroll;

  &__title {
    font-size: $font-md;
    color: var(--c-button-border);
    margin: $space-sm;
  }
}
</style>
