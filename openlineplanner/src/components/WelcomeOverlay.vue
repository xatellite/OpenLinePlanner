<template>
  <div v-if="isShown" class="welcome">
    <div class="welcome__container">
      <button @click="skipTour"  class="welcome__close button--transparent"><CloseIcon /></button>
      <div class="welcome__content">
        <div class="welcome__content__head">
          <span>Welcome to</span>
          <OLPLogo />
          <span>V {{ version }}</span>
        </div>
        <p class="welcome__content__desc">
          This tool allows you to shape the public transport system around you.
          <br><br>
          OpenLinePlanner is a tool for prototyping public transport lines. 
          For this, it uses OpenStreetMap Data to estimate the population in a planning area, thus enabling you to create a data-driven planning based on the coverage area that a public transit line may have.
          <br><br>
          Let's take the tour to learn how it works, or dive right in if you are already familiar with the tool.
        </p>
        <div class="welcome__content__actions">
          <button class="button--fit" @click="skipTour">Skip Tour</button>
          <button class="button--fit button--accent" @click="takeTour">Take Tour</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import CloseIcon from "vue-material-design-icons/Close.vue";
import { useUIStore } from "../stores/ui";
import OLPLogo from "./OLPLogo.vue";
const overlayVersion = "0.7"

export default {
  components: { CloseIcon, OLPLogo },
  data() {
    return {
      version: import.meta.env.VITE_VERSION ? import.meta.env.VITE_VERSION : "0.0.dev",
      uiStore: useUIStore(),
      isShown: true, //!(localStorage.getItem("welcomeOverlayShown") === `true-${overlayVersion}`)
    };
  },
  mounted() {
    window.addEventListener("showWelcome", () => {
      this.isShown = true;
    });
  },
  methods: {
    skipTour() {
      localStorage.setItem("welcomeOverlayShown", `true-${overlayVersion}`);
      this.isShown = false;
    },
    takeTour() {
      localStorage.setItem("welcomeOverlayShown", `true-${overlayVersion}`);
      this.isShown = false;
      window.dispatchEvent(new Event("startTour"));
    }
  }
}
</script>

<style lang="scss">
.welcome {
  position: fixed;
  top: 0;
  left: 0;
  z-index: 10;
  width: 100vw;
  height: 100vh;
  background-color: rgba(0, 0, 0, 0.3);
  display: flex;
  align-items: center;
  justify-content: center;

  &__container {
    background-color: var(--c-background-primary);
    border-radius: $br-md;
    max-width: 400px;
    padding: $space-sm;
    width: 90%;
    display: flex;
    flex-direction: column;
  }

  &__close {
    align-self: flex-end;
  }

  &__content {
    display: flex;
    flex-direction: column;
    align-items: center;

    &__head {
      display: flex;
      flex-direction: column;
      align-items: center;
      margin-bottom: $space-sm;
      font-size: $font-lg;

      & > * {
        margin: 0;
      }
    }

    &__desc {
      text-align: center;
      margin: $space-sm $space-lg;
      width: 300px;
    }

    &__actions {
      display: flex;
      justify-content: space-between;
      padding: $space-md $space-sm;
      box-sizing: border-box;
      width: 100%;
    }
  }
}
</style>