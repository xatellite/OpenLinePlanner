<template>
  <main>
    <InteractiveMap />
    <div class="main-menu-pane box">
      <LineList />
    </div>
    <ViewSettings />
  </main>
  <ExportLoader />
  <PageFooter />
</template>

<script>
import InteractiveMap from "@/components/InteractiveMap.vue";
import PageFooter from "@/components/PageFooter.vue";
import LineList from "@/components/LineList.vue";
import ExportLoader from "@/components/ExportLoader.vue";
import ViewSettings from "@/components/ViewSettings.vue";

export default {
  components: {
    InteractiveMap,
    PageFooter,
    LineList,
    ExportLoader,
    ViewSettings,
  },
};
</script>

<style lang="scss">
.main-menu-pane {
  position: absolute;
  z-index: 10;
  top: 0;
  left: 0;
  margin: $space-md;
  display: flex;
  flex-direction: row;

  @media (max-width: 700px), (max-height: 600px) {
    position: relative;
    margin-top: -100px;
  }
}

main {
  display: block;

  @media (max-width: 700px), (max-height: 600px) {
    display: flex;
    flex-direction: column;
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
