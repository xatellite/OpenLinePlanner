<template>
  <div class="page-header">
    <div class="page-header__center-box">
      <h1 class="page-header__title"><img class="page-header__title_logo" src="@/assets/logo.svg"/>OpenLinePlanner</h1>
    </div>
    <nav class="page-header__navigation">
      <router-link to="/" class="page-header__navigation__item">Planning</router-link>
      <router-link to="/timetable" class="page-header__navigation__item">Timetable</router-link>
      <router-link to="/project" class="page-header__navigation__item">Project</router-link>
    </nav>
    <GithubCorner />
  </div>
</template>

<script>
import GithubCorner from "@/components/GithubCorner.vue";
import { RouterLink} from "vue-router";
export default {
  components: {GithubCorner, RouterLink },
};
</script>

<style lang="scss" scoped>
.page-header {
  display: flex;
  align-items: center;
  padding: 0;

  // @media (max-width: 700px), (max-height: 600px) {
  //   width: 100%;
  // }

  &__center-box {
    display: flex;
    flex-direction: column;
    align-items: center;
    margin-bottom: $space-sm;
  }

  &__title {
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

  &__navigation {
    display: flex;
    gap: $space-md;
    margin-left: $space-lg;
    font-weight: bold;

    .router-link-active {
      border-bottom: 5px solid $c-accent-three;
      border-radius: 2px;
    }
  }
}
</style>