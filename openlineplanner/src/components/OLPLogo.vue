<template>
  <div class="olp-logo">
    <img v-if="uiStore.mode == 'light'" class="olp-logo__logo" src="@/assets/logo.svg" />
    <img v-else class="olp-logo__logo" src="@/assets/logo-dark.svg" />
    <h1 :class="`olp-logo__title ${reduceOnMobile ? 'olp-logo__title--reduce-m' : ''}`">OpenLinePlanner</h1>
  </div>
</template>

<script>
import { useUIStore } from '../stores/ui';

export default {
  props: {
    reduceOnMobile: {
      type: Boolean,
      default: false,
    },
  },
  setup() {
    return {
      uiStore: useUIStore(),
    };
  },
}
</script>

<style lang="scss">
.olp-logo {
  display: flex;
  align-items: center;
  margin: $space-sm $space-ssm;
  padding: 6px 26px 4px;


  &__title {
    color: var(--c-accent-one);
    font-family: "Poppins";
    font-weight: 700;
    font-size: 28px;
    margin: 0;

    &--reduce-m {
      @media (max-width: 900px), (max-height: 600px) {
        display: none;
      }
    }
  }

  &__logo {
    height: 36px;
    vertical-align: middle;
    padding-right: 10px;
  }
}
</style>