<template>
  <div class="data-list">
    <ListContainer title="Loaded Data Layers">
      <!-- <div
        class="data-list__item"
        v-for="line in linesStore.lines"
        :key="`${line.name}-list-entry`"
      >
        <LineElement :line="line" />
      </div> -->
      <div class="data-list__item data-list__item__row">
        click map to add data layer
      </div>
    </ListContainer>
  </div>
</template>

<script>
import { useLinesStore } from "../stores/lines";
import { useEditStore } from "../stores/editing";
import ListContainer from "./ListContainer.vue";

export default {
  components: {
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
.data-list {
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
      padding: $space-ssm $space-ssm;
      text-align: center;
    }
    margin-bottom: $space-ssm;
    // border-bottom: 1px solid rgba($c-text-primary, 0.2);
  }

  &__title {
    padding: 6px 26px 4px;
    color: $c-accent-one;
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
    color: $c-text-primary;
    height: 0px;
    border: none;
    margin: 0;
    border-top: 1px solid rgba($c-text-primary, 0.2);
  }
}
</style>
