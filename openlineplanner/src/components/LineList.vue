<template>
  <div class="box line-list">
    <div class="line-list__center-box">
      <h1 class="line-list__title">Open-Line-Planner</h1>
      <a class="line-list__subtitle">Check Github to Contribute</a>
    </div>
    <hr />
    <div
      class="line-list__item"
      v-for="line in linesStore.lines"
      :key="`${line.name}-list-entry`"
    >
      <LineElement :line="line"/>
      <hr />
    </div>
    <div class="line-list__item__row">
      <button @click="addLine"><PlusIcon /></button>
      <span class="grow">Add new Line</span>
      <div />
    </div>
    <hr />
    <ActionToolbar />
  </div>
</template>

<script>
import PlusIcon from "vue-material-design-icons/Plus.vue";

import { useLinesStore } from "../stores/lines";
import { useEditStore } from "../stores/editing";
import ActionToolbar from "./ActionToolbar.vue";
import LineElement from "./LineElement.vue";

export default {
  components: {
    PlusIcon,
    ActionToolbar,
    LineElement,
},
  data() {
    return {
      linesStore: useLinesStore(),
      editStore: useEditStore(),
    };
  },
  methods: {
    addLine() {
      const lineRef = this.linesStore.addLine();
      this.editStore.isEditing = lineRef;
      this.editStore.isAdding = lineRef;
      this.editStore.isExtending = lineRef;
    },
  },
};
</script>

<style lang="scss">
@import "@/assets/variables.scss";

.line-list {
  margin: 0 $space-sm;
  padding-bottom: $space-sm;

  &__center-box {
    display: flex;
    flex-direction: column;
    align-items: center;
  }

  &__item {
    &__row {
      display: flex;
      align-items: center;
      padding: $space-ssm $space-sm;

      .grow {
        flex-grow: 1;
        padding: $space-ssm;
      }
    }
  }

  &__title {
    padding: 0 $space-sm;
    margin: $space-ssm $space-ssm 0;
  }

  hr {
    color: $c-text-primary;
    height: 0px;
    border: none;
    border-top: 1px solid rgba($c-text-primary, 0.2);
  }
}
</style>
