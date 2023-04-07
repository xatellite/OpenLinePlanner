<template>
  <div class="loading-overlay" v-if="dataStore.loadingLayer">
    <div class="box loading-overlay__content">
      <span class="loading-overlay__content__title">Loading data layer</span>
      <span class="loading-overlay__content__subtitle"
        >Stand by and get a coffee. This might take some time</span
      >
      <LoaderAnimation />
    </div>
  </div>
</template>

<script>
import { useDataStore } from "@/stores/data";
import LoaderAnimation from "./LoaderAnimation.vue";
export default {
  components: { LoaderAnimation },
  data() {
    return {
      dataStore: useDataStore(),
    };
  },
};
</script>

<style lang="scss">
.loading-overlay {
  position: absolute;
  top: 0;
  left: 0;
  display: flex;
  justify-content: center;
  align-items: center;
  width: 100%;
  height: 100%;
  background-color: rgba(0, 0, 0, 0.3);
  z-index: 10;

  &__content {
    box-shadow: $bs-md;
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: $space-md;

    &__title {
      font-size: $font-lg;
      font-weight: 700;
      padding: $space-sm 0;
    }
  }
}
</style>
