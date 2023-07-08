<template>
  <WelcomeOverlay />
  <WelcomeTour />
  <PageHeader />
  <RouterView />
  <PageFooter />
</template>

<script>
import WelcomeTour from "./components/WelcomeTour.vue";
import WelcomeOverlay from "./components/WelcomeOverlay.vue";
import { RouterView } from "vue-router";
import PageHeader from "@/components/PageHeader.vue";
import PageFooter from "@/components/PageFooter.vue";
import "@fontsource/poppins";
import "@fontsource/poppins/700.css";
import { useUIStore } from './stores/ui';

export default {
  components: {
    RouterView,
    PageHeader,
    PageFooter,
    WelcomeOverlay,
    WelcomeTour
},
  data() {
    return {
      uiStore: useUIStore(),
    }
  },
  mounted() {
    this.uiStore.getMapCenter();
  },
};
</script>

<!-- Not to be scoped, as the view is used to inject global scss -->
<style lang="scss">
#app{
  height: 100%;
  display: flex;
  flex-direction: column;
  max-height: 100%;
}
</style>
