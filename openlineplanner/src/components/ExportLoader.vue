<template>
  <div class="export" v-if="overlayStore.exporting">
    <span class="export__message">Exporting planning to PDF...</span>
    <LoadingIcon class="export__loader" />
  </div>
</template>

<script>
import LoadingIcon from "vue-material-design-icons/Loading.vue";
import { useOverlayStore } from '../stores/overlay';

export default {
  setup() {
    const overlayStore = useOverlayStore();
    return {
      overlayStore,
    };
  },
  components: {
    LoadingIcon,
  },
};
</script>


<style lang="scss" scoped>
@import "@/assets/variables.scss";

.export {
  position: absolute;
  z-index: 100;
  top: 0;
  left: 0;
  display: flex;
  height: 100%;
  width: 100%;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  background-color: rgba($c-text-primary, 0.3);
  color: $c-text-inverted;

  &__message {
    padding-bottom: $space-md;
    font-size: $font-lg;
    font-weight: 700;
  }

  &__loader {
    -webkit-animation: infiniteRotate 2s linear infinite;
    animation: infiniteRotate 2s linear infinite;
    padding: 0;
  }
}


/* Safari 4.0 - 8.0 */
@-webkit-keyframes infiniteRotate {
  0% {
    -webkit-transform: rotate(0deg);
  }
  100% {
    -webkit-transform: rotate(360deg);
  }
}
/* Standard syntax */
@keyframes infinite-rotate {
  0% {
    -webkit-transform: rotate(0deg);
  }
  100% {
    -webkit-transform: rotate(360deg);
  }
}

</style>