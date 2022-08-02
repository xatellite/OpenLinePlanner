<template>
  <div class="footer">
    <span>This tool was created as part of the interdisciplinary project  of the master class Rail Technology and Management of Railway Systems @FH-St.Pölten</span>
    <span class="footer__center">V0.4</span>
    <span class="footer__right">Designed and Built by @TheNewCivilian, Map by ©Mapbox ©OpenStreetMap</span>
  </div>
</template>

<script>
export default {};
</script>

<style lang="scss">
.footer {
  position: absolute;
  bottom: 0;
  z-index: 100;
  background: $c-text-primary;
  color: $c-text-inverted;
  font-size: $space-sm;
  width: 100%;
  display: grid;
  grid-template-columns: 1fr $space-lg 1fr;
  padding: 4px $space-sm;
  box-sizing: border-box;

  &__center {
    font-weight: 700;
    text-align: center;
  }

  &__right {
    text-align: right;
  }

}
</style>
