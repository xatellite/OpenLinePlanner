<template>
  <div class="footer">
    <span class="footer__left">OpenLinePlanner is a product for all! <GithubCorner /></span>
    <span class="footer__center">{{version}} - {{ majorName }}</span>
    <span class="footer__right">Launched by @xatellite, Supported by FH St. Pölten</span>
  </div>
</template>

<script>
import GithubCorner from './GithubCorner.vue';
export default {
  components: { GithubCorner },
  setup() {
    return {
      version: import.meta.env.VITE_VERSION,
      majorName: import.meta.env.VITE_MAJORNAME,
    };
  },
};
</script>

<style lang="scss" scoped>
.footer {
  background: var(--c-background-inverted);
  color: var(--c-text-inverted--alt);
  font-size: $space-sm;
  width: 100%;
  display: flex;
  justify-content: space-between;
  grid-template-columns: 1fr 0.1fr 1fr;
  padding: 4px $space-sm;
  box-sizing: border-box;

  & > * {
    flex-grow: 1;
    width: 0;
  }


  &__center {
    font-weight: 700;
    text-align: center;
  }

  &__left {
    display: flex;
    align-items: center;
    gap: $space-sm;
  }

  &__right {
    text-align: right;
  }

  @media (max-width: 700px), (max-height: 600px) {
    display: flex;
    flex-direction: column;

    & > * {
      text-align: center;
    }
  }
}
</style>
