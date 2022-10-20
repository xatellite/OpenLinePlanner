<template>
  <div class="line-analysis-list">
    <LineAnalysisEntry
      v-for="(line, index) in lineStore.lines"
      :line="line"
      :key="`${index}-line`"
    />
  </div>
</template>

<script>
import { useLinesStore } from "../stores/lines";
import LineAnalysisEntry from "./LineAnalysisEntry.vue";
export default {
  data() {
    return {
      lineStore: useLinesStore(),
    };
  },
  components: { LineAnalysisEntry },
};
</script>

<style lang="scss" scoped>
.line-analysis-list {
  display: flex;
  flex-direction: column;
  margin-bottom: $space-lg;
}
</style>
