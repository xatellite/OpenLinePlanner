<template>
  <main>
    <InteractiveMap />
    <div class="main-menu-pane">
      <Logo />
      <LineList />
    </div>
    <ViewSettings />
  </main>
  <div class="mobile">
    <h1>OpenLinePlanner is not yet supporting mobile use!</h1>
    <span>Please use your web browser or tablet to start planning</span>
  </div>
  <ExportLoader />
  <PageFooter />
</template>

<script>
import InteractiveMap from "@/components/InteractiveMap.vue";
import PageFooter from "@/components/PageFooter.vue";
import Logo from "@/components/PageLogo.vue";
import LineList from "@/components/LineList.vue";
import ExportLoader from "@/components/ExportLoader.vue";
import ViewSettings from "@/components/ViewSettings.vue";

export default {
  components: {
    InteractiveMap,
    PageFooter,
    Logo,
    LineList,
    ExportLoader,
    ViewSettings,
  },
};
</script>


<style lang="scss">
@import "@/assets/variables.scss";
.main-menu-pane {
  position: absolute;
  z-index: 10;
  top: 0;
  left: 0;
  margin: $space-md;
  display: flex;
  flex-direction: row;
}

main {
  display: block;

  @media (max-width: 700px), (max-height: 600px) {
    display: none;
  }
}

.mobile {
  display: none;
  @media (max-width: 700px), (max-height: 600px) {
    display: flex;
    flex-direction: column;
    height: 100vh;
    align-items: center;
    justify-content: center;
    padding: $space-md;
  }
}
</style>
