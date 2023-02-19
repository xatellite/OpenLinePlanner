<template>
  <div class="footer">
    <span>This tool was created as part of the interdisciplinary project  of the master class Rail Technology and Management of Railway Systems @FH-St.Pölten</span>
    <span class="footer__center">V0.6</span>
    <span class="footer__right">Map by ©Mapbox ©OpenStreetMap</span>
  </div>
</template>

<script>
export default {};
</script>

<style lang="scss">
.footer {
  position: fixed;
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

  @media (max-width: 700px), (max-height: 600px) {
    position: relative;
  }

  &__center {
    font-weight: 700;
    text-align: center;
    margin: 0 $space-lg;
  }

  &__right {
    text-align: right;
  }
}
</style>
