<template>
  <div class="navigation-bar">
    <router-link class="button" to="/">
      <ArrowUpLeftBoldIcon />
    </router-link>
    <div class="navigation-bar__title">
      <h1>Open-Line-Planner</h1>
      <span>Line Overview</span>
    </div>
    <div />
  </div>
</template>

<script>
import ArrowUpLeftBoldIcon from "vue-material-design-icons/ArrowUpLeftBold.vue";
export default {
  components: {
    ArrowUpLeftBoldIcon,
  },
};
</script>

<style lang="scss">
.navigation-bar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0 $space-md;
  border-bottom: 1px solid rgba($c-text-primary, 0.2);

  &__title {
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: $space-sm;

    h1 {
      margin: 0;
    }
  }
}
</style>