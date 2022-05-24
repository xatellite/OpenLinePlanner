<template>
  <div class="footer">
    <span>This tool was created as part of the interdisciplinary project  of the master class Rail Technology and Management of Railway Systems @FH-St.Pölten</span>
    <span>V0.1</span>
    <span>Designed and Built by @TheNewCivilian</span>
  </div>
</template>

<script>
export default {};
</script>

<style lang="scss">
@import "@/assets/variables.scss";
.footer {
  background: $c-text-primary;
  color: $c-text-inverted;
  font-size: $space-sm;
  height: $space-md;
  width: 100%;
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0 $space-sm;
  box-sizing: border-box;

  * > {
    flex-grow: 1;
  }
}
</style>
