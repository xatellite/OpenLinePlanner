<template>
  <div class="line-list">
    <div class="line-list__center-box">
      <h1 class="line-list__title">OpenLinePlanner</h1>
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
      <div class="line-list__item line-list__item__row">
        <button @click="addLine"><PlusIcon /></button>
        <span class="grow">Add new Line</span>
        <div />
      </div>
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
      // Matomo Tracking
      if (window && window.Piwik) {
        window.Piwik.getTracker().trackEvent("editing", "addLine");
      }
      const line = this.linesStore.addLine();
      this.editStore.isEditing = line;
      this.editStore.isExtending = -1;
    },
  },
};
</script>

<style lang="scss">
.line-list {
  // margin: 0 $space-sm;
  padding-bottom: $space-sm;

  @media (max-width: 700px), (max-height: 600px) {
    width: 100%;
  }

  &__center-box {
    display: flex;
    flex-direction: column;
    align-items: center;
    margin-bottom: $space-sm;
  }

  &__container {
    max-height: 300px;
    overflow-y: auto;
    background-color: darken($c-box, 2);

    @media (max-width: 700px), (max-height: 600px) {
      max-height: none;
    }
  }

  &__item {
    background-color: $c-box;
    margin: $space-sm;
    border: 1px solid $c-button-border;
    border-radius: $br-md;
    overflow: hidden;
    &__row {
      display: flex;
      align-items: center;
      padding: $space-ssm $space-ssm;

      .grow {
        flex-grow: 1;
        padding: $space-ssm;
      }
    }
    margin-bottom: $space-ssm;
    // border-bottom: 1px solid rgba($c-text-primary, 0.2);
  }

  &__title {
    padding: 0 $space-sm;
    margin: $space-ssm $space-ssm 0;
  }

  hr {
    color: $c-text-primary;
    height: 0px;
    border: none;
    margin: 0;
    border-top: 1px solid rgba($c-text-primary, 0.2);
  }
}

.line-list__title {
  padding: 6px 26px 4px;
  color: $c-accent-one;
  font-family: "Poppins";
  font-weight: 700;
  font-size: 28px;
}
</style>
