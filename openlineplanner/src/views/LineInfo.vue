<template>
  <div class="line-info">
    <NavigationBar />
    <LineAnalysisList />
    <PageFooter />
  </div>
</template>

<script>
import NavigationBar from '../components/NavigationBar.vue';
import PageFooter from '../components/PageFooter.vue';
import LineAnalysisList from '../components/LineAnalysisList.vue';
export default {
    components: { NavigationBar, PageFooter, LineAnalysisList }
}
</script>

<style lang="scss">
.line-info {
  height: 100%;
}
</style>
