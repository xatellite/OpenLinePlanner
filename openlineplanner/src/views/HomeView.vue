<script setup>
import InteractiveMap from "@/components/InteractiveMap.vue";
import Footer from "@/components/PageFooter.vue";
import Logo from "@/components/PageLogo.vue";
import LineList from "@/components/LineList.vue";
</script>

<template>
  <main>
    <InteractiveMap />
    <div class="main-menu-pane">
      <Logo />
      <LineList />
    </div>
  </main>
  <Footer />
</template>

<style lang="scss">
@import "@/assets/variables.scss";
.main-menu-pane {
  position: absolute;
  top: 0;
  left: 0;
  margin: $space-md;
  display: flex;
  flex-direction: row;
}
</style>
