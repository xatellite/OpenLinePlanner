<template>
  <div class="navigation-bar">
    <router-link class="button" to="/">
      <ArrowLeftIcon />
    </router-link>
    <div class="navigation-bar__title">
      <h1 class="navigation-bar__title-text"><img class="navigation-bar__title_logo" src="@/assets/logo.svg"/>OpenLinePlanner</h1>
      <h2 class="navigation-bar__title-text-subtext">Timetable</h2>
    </div>
    <div />
  </div>
</template>

<script>
import ArrowLeftIcon from "vue-material-design-icons/ArrowLeftBold.vue";
export default {
  components: {
    ArrowLeftIcon,
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

  &__title-text-subtext {
    padding: 0px 0px 12px;
    margin: 0;
  }

  &__title {
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: $space-sm;

    h1 {
      margin: 0;
    }
  }

  &__title-text {
    padding: 6px 26px 4px;
    color: $c-accent-one;
    font-family: "Poppins";
    font-weight: 700;
    font-size: 28px;
    margin: $space-ssm $space-ssm 0;
  }

  &__title_logo {
    height: 36px;
    vertical-align: middle;
    padding-right: 10px;
  }
}
</style>