<template>
  <div class="page-header" id="page-header">
    <OLPLogo :reduceOnMobile="true" />
    <button class="mobile-nav" @click="menuActive = !menuActive">
      {{ $route.name }} <MenuDownIcon />
    </button>
    <nav :class="`page-header__navigation ${menuActive ? 'active' : ''}`">
      <router-link to="/" class="page-header__navigation__item"
        >Data</router-link
      >
      <router-link to="/planning" id="nav-planning" class="page-header__navigation__item"
        >Planning</router-link
      >
      <router-link to="/timetable" id="nav-timetable" class="page-header__navigation__item"
        >Timetable</router-link
      >
      <router-link to="/project" id="nav-project" class="page-header__navigation__item"
        >Project</router-link
      >
    </nav>
    <div class="page-header__settings">
      <button @click="uiStore.toggleMode" class=" button--transparent">
        <WhiteBalanceSunnyIcon v-if="uiStore.mode == 'light'"/>
        <WeatherNightIcon v-else/>
      </button>
      <button @click="handleInfo" class="button--transparent">
        <InformationOutlineIcon />
      </button>
    </div>
  </div>
</template>

<script>
import GithubCorner from "@/components/GithubCorner.vue";
import MenuDownIcon from "vue-material-design-icons/MenuDown.vue";
import WhiteBalanceSunnyIcon from "vue-material-design-icons/WhiteBalanceSunny.vue";
import WeatherNightIcon from "vue-material-design-icons/WeatherNight.vue";
import InformationOutlineIcon from "vue-material-design-icons/InformationOutline.vue";
import OLPLogo from "./OLPLogo.vue";

import { RouterLink } from "vue-router";
import { useUIStore } from '../stores/ui';

export default {
  components: { GithubCorner, RouterLink, MenuDownIcon, WhiteBalanceSunnyIcon, WeatherNightIcon, OLPLogo, InformationOutlineIcon },
  data() {
    return {
      menuActive: false,
      uiStore: useUIStore(),
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
    handleInfo() {
      window.dispatchEvent(new Event("showWelcome"));
    }
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

  .router-link-active {
    position: relative;
    // border-bottom: 5px solid var(--c-accent-three);

    &::after {
      content: "";
      position: absolute;
      bottom: -4px;
      left: 0;
      width: 100%;
      height: 5px;
      border-radius: 5px;
      background-color: var(--c-accent-three);
    }
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
    align-items: center;
    gap: $space-md;
    margin-left: $space-lg;
    font-weight: 600;

    @media (max-width: 700px), (max-height: 600px) {
      display: none;

      & > * {
        text-align: center;
        padding: $space-sm;
        border-bottom: 1px solid var(--c-button-border);
      }

      .router-link-active {
        display: none;
      }
    }

    &__button {
      width: auto;
      height: auto;
      padding: 4px $space-sm;
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
      background-color: var(--c-box);
      width: 100%;
      gap: 0;
      border-top: 1px solid var(--c-button-border);
    }
  }

  &__settings {
    margin: 0  $space-sm 4px auto;
    display: flex;
    gap: $space-sm;
  }
}
</style>
