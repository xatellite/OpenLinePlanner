<template>
  <div class="page-header" id="page-header">
    <div class="page-header__box">
      <img class="page-header__logo" src="@/assets/logo.svg" />
      <h1 class="page-header__title">OpenLinePlanner</h1>
    </div>
    <button class="mobile-nav" @click="menuActive = !menuActive">
      {{ $route.name }} <MenuDownIcon />
    </button>
    <nav :class="`page-header__navigation ${menuActive ? 'active' : ''}`">
      <router-link to="/" class="page-header__navigation__item"
        >Project</router-link
      >
      <router-link to="/data" class="page-header__navigation__item"
        >Data</router-link
      >
      <router-link to="/planning" class="page-header__navigation__item"
        >Planning</router-link
      >
      <router-link to="/timetable" class="page-header__navigation__item"
        >Timetable</router-link
      >
    </nav>
    <GithubCorner />
  </div>
</template>

<script>
import GithubCorner from "@/components/GithubCorner.vue";
import MenuDownIcon from "vue-material-design-icons/MenuDown.vue";
import { RouterLink } from "vue-router";
export default {
  components: { GithubCorner, RouterLink, MenuDownIcon },
  data() {
    return {
      menuActive: false,
    };
  },
  mounted() {
    window.addEventListener("click", this.handleClickOutside);
    this.$watch(
      () => this.$route.path,
      () => {
        this.menuActive = false;
      }
    );
  },
  methods: {
    handleClickOutside(e) {
      let refElement = e.target;
      while (refElement.parentElement != null) {
        if (refElement.id === "page-header") {
          return;
        }
        refElement = refElement.parentElement;
      }
      this.menuActive = false;
    },
  },
  beforeUnmount() {
    window.removeEventListener("click", this.handleClickOutside);
  },
};
</script>

<style lang="scss" scoped>
.page-header {
  position: relative;
  display: flex;
  align-items: center;
  padding: 0;

  @media (max-width: 700px), (max-height: 600px) {
    width: 100%;
  }

  &__box {
    display: flex;
    align-items: center;
    margin: $space-sm $space-ssm;
    padding: 6px 26px 4px;
  }

  &__title {
    color: $c-accent-one;
    font-family: "Poppins";
    font-weight: 700;
    font-size: 28px;
    margin: 0;

    @media (max-width: 700px), (max-height: 600px) {
      display: none;
    }
  }

  &__logo {
    height: 36px;
    vertical-align: middle;
    padding-right: 10px;
  }

  .router-link-active {
    border-bottom: 5px solid $c-accent-three;
  }

  .mobile-nav {
    display: none;
    font-weight: bold;
    cursor: pointer;
    font-size: $font-md;
    background-color: transparent;
    width: auto;
    border: none;
    @media (max-width: 700px), (max-height: 600px) {
      display: flex;
    }
  }

  &__navigation {
    display: flex;
    gap: $space-md;
    margin-left: $space-lg;
    font-weight: bold;

    @media (max-width: 700px), (max-height: 600px) {
      display: none;

      & > * {
        text-align: center;
        padding: $space-sm;
        border-bottom: 1px solid $c-button-border;
      }

      .router-link-active {
        display: none;
      }
    }
  }

  @media (max-width: 700px), (max-height: 600px) {
    .active {
      display: flex;
      position: absolute;
      top: 100%;
      left: 0;
      margin-left: 0;
      z-index: 100;
      flex-direction: column;
      background-color: $c-box;
      width: 100%;
      gap: 0;
      border-top: 1px solid $c-button-border;
    }
  }
}
</style>
