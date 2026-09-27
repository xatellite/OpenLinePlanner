<template>
  <div class="data-list">
    <ListContainer title="Loaded Data Layers">
      <div
        class="data-list__item"
        v-for="layer in dataStore.layers"
        :key="`${layer.name}-list-entry`"
      >
        <DataSourcesListEntry :layer="layer" />
      </div>
      <div class="data-list__item data-list__item__row">
        click map to add data layer
      </div>
      <div class="data-list__setup">
        <button
          id="load-region-data"
          class="button--fit button--accent"
          @click="openRegionSetup"
        >
          <DownloadIcon />
          Load region data
        </button>
        <span class="data-list__setup__hint">
          Missing your area? Download OpenStreetMap data for a new region.
        </span>
      </div>
    </ListContainer>
  </div>
</template>

<script>
import DownloadIcon from "vue-material-design-icons/Download.vue";
import ListContainer from "./ListContainer.vue";
import DataSourcesListEntry from "./DataSourcesListEntry.vue";
import { useDataStore } from '../stores/data';

export default {
  components: {
    ListContainer,
    DataSourcesListEntry,
    DownloadIcon
  },
  data() {
    return {
      dataStore: useDataStore(),
    };
  },
  methods: {
    openRegionSetup() {
      window.dispatchEvent(new Event("showRegionSetup"));
    },
  },
};
</script>

<style lang="scss" scoped>
.data-list {
  height: 100%;
  width: 30%;
  min-width: 320px;
  min-height: 400px;
  @media (max-width: 700px), (max-height: 600px) {
    width: 100%;
  }

  &__setup {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: $space-ssm;
    margin: $space-sm;
    padding-top: $space-sm;
    border-top: 1px solid var(--c-button-border);

    button {
      display: flex;
      align-items: center;
      gap: $space-ssm;
      width: auto;
      padding: $space-ssm $space-sm;
    }

    &__hint {
      font-size: $font-sm;
      text-align: center;
    }
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
