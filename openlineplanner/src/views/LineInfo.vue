<template>
  <div>
    <NavigationBar />
    <div>
      <LineAnalysisEntry />
    </div>
    <PageFooter />
  </div>
</template>

<script>
import NavigationBar from '../components/NavigationBar.vue';
import PageFooter from '../components/PageFooter.vue';
import LineAnalysisEntry from '../components/LineAnalysisEntry.vue';
export default {
    components: { NavigationBar, PageFooter, LineAnalysisEntry }
}
</script>

<style>

</style>