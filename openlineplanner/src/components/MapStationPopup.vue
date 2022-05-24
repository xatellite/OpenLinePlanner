<template>
  <div class="box station-popup">
    <span>Station</span>
  </div>
</template>

<script>
export default {
  props: {
    point: Object,
  },
};
</script>

<style lang="scss">
@import "@/assets/variables.scss";
.add-station {
  position: absolute;
  top: -100px;
  display: flex;
}
</style>