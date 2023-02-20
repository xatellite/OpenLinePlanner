<template>
  <svg
    class="icon-color"
    width="25"
    height="25"
    viewBox="0 0 20 18"
    fill="none"
    xmlns="http://www.w3.org/2000/svg"
  >
    <circle
      cx="10" cy="8" r="8"
      :fill="color"
    />
  </svg>
</template>

<script>
export default {
  props: {
    color: String,
  },
};
</script>

<style lang="scss" scoped>
.icon-color {
  padding: 0 8px;
  :hover {
    cursor: pointer;
  }
  circle:hover {
    stroke-width: 2px;
    stroke: $c-accent-one;
  }
}

</style>
