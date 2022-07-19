<template>
  <div class="box line-list">
    <div class="line-list__center-box">
      <h1 class="line-list__title">Open-Line-Planner</h1>
      <a
        class="line-list__subtitle"
        href="https://github.com/TheNewCivilian/OpenLinePlanner"
        >Contribute on Github</a
      >
    </div>
    <hr />
    <div class="line-list__container">
      <div
        class="line-list__item"
        v-for="line in linesStore.lines"
        :key="`${line.name}-list-entry`"
      >
        <LineElement :line="line" />
      </div>
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
      const line = this.linesStore.addLine();
      this.editStore.isEditing = line;
      this.editStore.isExtending = -1;
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

  &__container {
    max-height: 500px;
    overflow: auto;
  }

  &__item {
    &__row {
      display: flex;
      align-items: center;
      padding: $space-ssm $space-ssm;

      .grow {
        flex-grow: 1;
        padding: $space-ssm;
      }
    }
    border-bottom: 1px solid rgba($c-text-primary, 0.2);
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
