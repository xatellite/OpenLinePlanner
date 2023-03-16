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
  border: 1px solid $c-button-border;
  border-radius: $br-md;
  background-color: darken($c-box, 2);
  height: 100%;
  width: 100%;
  max-height: 100%;
  overflow: scroll;

  &__title {
    font-size: $font-md;
    color: $c-button-border;
    margin: $space-sm;
  }
}
</style>
