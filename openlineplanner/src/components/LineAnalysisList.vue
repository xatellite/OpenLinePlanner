<template>
  <div class="line-analysis-list">
    <ListContainer title="Planned Lines">
      <LineAnalysisEntry
        v-for="(line, index) in lineStore.lines"
        :line="line"
        :key="`${index}-line`"
      />
    </ListContainer>
  </div>
</template>

<script>
import { useLinesStore } from "../stores/lines";
import LineAnalysisEntry from "./LineAnalysisEntry.vue";
import ListContainer from "./ListContainer.vue";
export default {
  data() {
    return {
      lineStore: useLinesStore(),
    };
  },
  components: { LineAnalysisEntry, ListContainer },
};
</script>

<style lang="scss" scoped>
.line-analysis-list {
  flex-grow: 1;
}
</style>
