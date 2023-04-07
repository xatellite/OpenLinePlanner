<template>
  <div class="data-list">
    <ListContainer title="Loaded Data Layers">
      <div
        class="data-list__item"
        v-for="layer in dataStore.layers"
        :key="`${layer.properties.id}-list-entry`"
      >
        <DataSourcesListEntry :layer="layer" />
      </div>
      <div class="data-list__item data-list__item__row">
        click map to add data layer
      </div>
    </ListContainer>
  </div>
</template>

<script>
import ListContainer from "./ListContainer.vue";
import DataSourcesListEntry from "./DataSourcesListEntry.vue";
import { useDataStore } from '../stores/data';

export default {
  components: {
    ListContainer,
    DataSourcesListEntry
},
  data() {
    return {
      dataStore: useDataStore(),
    };
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
      padding: $space-ssm $space-ssm;
      text-align: center;
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
