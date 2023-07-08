<template>
  <div class="footer">
    <span class="footer__left">OpenLinePlanner is a product for all! <GithubCorner /></span>
    <span class="footer__center">V0.7-00</span>
    <span class="footer__right">Launched by @xatellite, Supported by FH St. Pölten</span>
  </div>
</template>

<script>
import GithubCorner from './GithubCorner.vue';
export default {
  components: { GithubCorner },};
</script>

<style lang="scss" scoped>
.footer {
  background: var(--c-background-inverted);
  color: var(--c-text-inverted--alt);
  font-size: $space-sm;
  width: 100%;
  display: grid;
  grid-template-columns: 1fr $space-lg 1fr;
  padding: 4px $space-sm;
  box-sizing: border-box;

  &__center {
    font-weight: 700;
    text-align: center;
    margin: 0 $space-lg;
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
