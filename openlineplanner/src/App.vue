<template>
  <PageHeader />
  <RouterView />
  <PageFooter />
</template>

<script setup>
import { RouterView } from "vue-router";
import PageHeader from "@/components/PageHeader.vue";
import PageFooter from "@/components/PageFooter.vue";
import "@fontsource/poppins";
import "@fontsource/poppins/700.css";
</script>

<!-- Not to be scoped, as the view is used to inject global scss -->
<style lang="scss">
#app{
  height: 100%;
  display: flex;
  flex-direction: column;
  max-height: 100%;
}
</style>
