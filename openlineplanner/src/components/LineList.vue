<template>
  <div class="line-list">
    <ListContainer title="Planned Lines">
      <div
        class="line-list__item"
        v-for="line in linesStore.lines"
        :key="`${line.name}-list-entry`"
      >
        <LineElement :line="line" />
      </div>
      <div id="add-line-entry" class="line-list__item line-list__item__row">
        <button @click="addLine"><PlusIcon /></button>
        <span class="grow">Add new Line</span>
        <div />
      </div>
    </ListContainer>
  </div>
</template>

<script>
import PlusIcon from "vue-material-design-icons/Plus.vue";

import { useLinesStore } from "../stores/lines";
import { useEditStore } from "../stores/editing";
import LineElement from "./LineElement.vue";
import ListContainer from "./ListContainer.vue";

export default {
  components: {
    PlusIcon,
    LineElement,
    ListContainer,
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

<style lang="scss" scoped>
.line-list {
  height: 100%;
  width: 30%;
  min-width: 320px;
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
    max-height: 70vh;
    overflow-y: auto;
    background-color: var(--c-box-darkened);

    @media (max-width: 700px), (max-height: 600px) {
      max-height: none;
    }
  }

  &__item {
    background-color: var(--c-box);
    margin: $space-sm;
    border: 1px solid var(--c-button-border);
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
    // border-bottom: 1px solid rgba(var(--c-primary-light), 0.2);
  }

  &__title {
    padding: 6px 26px 4px;
    color: var(--c-accent-one);
    font-family: "Poppins";
    font-weight: 700;
    font-size: 28px;
    margin: $space-ssm $space-ssm 0;
  }

  &__title_logo {
    height: 36px;
    vertical-align: middle;
    padding-right: 10px;
  }

  hr {
    color: var(--c-primary-light);
    height: 0px;
    border: none;
    margin: 0;
    border-top: 1px solid rgba(var(--c-primary-light), 0.2);
  }
}
</style>
