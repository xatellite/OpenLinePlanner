<template>
  <ModalDialog :shown="isShown" title="Load region data" :onClose="close">
    <p class="region-setup__intro">
      OpenLinePlanner needs OpenStreetMap data for the area you want to plan in.
      Pick an area below and it will be downloaded and prepared for you.
    </p>

    <div v-if="store.error" class="region-setup__error">{{ store.error }}</div>

    <!-- Step 1: where -->
    <div v-if="store.step === 'location'" class="region-setup__step">
      <form class="region-setup__search" @submit.prevent="store.search()">
        <TextInput
          title="Search for a place"
          placeholder="e.g. Vaduz, Liechtenstein"
          :value="store.searchQuery"
          :callback="(value) => (store.searchQuery = value)"
        />
        <button
          class="button--fit button--accent"
          type="submit"
          :disabled="store.searching || !store.searchQuery"
        >
          {{ store.searching ? "Searching..." : "Search" }}
        </button>
      </form>

      <ListContainer v-if="store.searchResults.length" title="Results">
        <button
          class="region-setup__entry button--transparent"
          v-for="result in store.searchResults"
          :key="`place-${result.place_id}`"
          @click="store.selectPoint({ lat: result.lat, lng: result.lon })"
        >
          <MapMarkerIcon />
          <span class="region-setup__entry__text">{{ result.display_name }}</span>
        </button>
      </ListContainer>

      <p class="region-setup__hint">
        You can also close this dialog and click anywhere on the map to pick a
        location.
      </p>
    </div>

    <!-- Step 2: which area -->
    <div v-if="store.step === 'area'" class="region-setup__step">
      <div v-if="store.loadingAreas" class="region-setup__loading">
        <LoaderAnimation />
        <span>Looking up administrative areas...</span>
      </div>
      <ListContainer v-else-if="store.areas.length" title="Available areas">
        <button
          class="region-setup__entry button--transparent"
          v-for="(area, index) in store.areas"
          :key="`area-${index}`"
          @click="store.selectArea(area)"
        >
          <span class="region-setup__entry__text">
            <span class="region-setup__entry__title">
              {{ areaName(area) }}
            </span>
            <span>{{ getLevelDescription(area.properties.admin_level) }}</span>
          </span>
          <span v-if="isImported(area)" class="region-setup__ready">ready</span>
        </button>
      </ListContainer>
      <div class="region-setup__actions">
        <button class="button--fit" @click="store.back()">Back</button>
      </div>
    </div>

    <!-- Step 3: confirm the download -->
    <div v-if="store.step === 'confirm'" class="region-setup__step">
      <div v-if="store.resolving" class="region-setup__loading">
        <LoaderAnimation />
        <span>Finding the best data source...</span>
      </div>
      <template v-else-if="store.resolved">
        <div class="region-setup__summary">
          <span class="region-setup__summary__row">
            <span>Area</span><strong>{{ areaName(store.selectedArea) }}</strong>
          </span>
          <span class="region-setup__summary__row">
            <span>Data source</span><strong>{{ store.resolved.name }}</strong>
          </span>
          <span class="region-setup__summary__row">
            <span>Download size</span>
            <strong>{{ formatSize(store.resolved.size_bytes) }}</strong>
          </span>
        </div>
        <p v-if="store.resolved.already_available" class="region-setup__hint">
          This area is already prepared - you can start planning right away.
        </p>
        <p v-else class="region-setup__hint">
          The download covers a larger region than your area; everything outside
          it is discarded afterwards. This can take several minutes and needs a
          few gigabytes of memory on the server. You can keep using the app while
          it runs.
        </p>
      </template>
      <div class="region-setup__actions">
        <button class="button--fit" @click="store.back()">Back</button>
        <button
          class="button--fit button--accent"
          :disabled="store.resolving || !store.resolved"
          @click="store.startImport()"
        >
          {{ store.resolved && store.resolved.already_available ? "Continue" : "Start download" }}
        </button>
      </div>
    </div>

    <!-- Step 4: progress -->
    <div v-if="store.step === 'progress'" class="region-setup__step">
      <div class="region-setup__summary">
        <span class="region-setup__summary__row">
          <span>Area</span><strong>{{ store.job && store.job.area_name }}</strong>
        </span>
        <span class="region-setup__summary__row">
          <span>Status</span><strong>{{ phaseLabel }}</strong>
        </span>
      </div>

      <div v-if="showProgressBar" class="region-setup__progress">
        <div
          class="region-setup__progress__bar"
          :style="{ width: downloadPercent + '%' }"
        ></div>
      </div>
      <span v-if="showProgressBar" class="region-setup__hint">
        {{ formatSize(store.job.downloaded_bytes) }} of
        {{ formatSize(store.job.total_bytes) }}
      </span>

      <div v-if="isFailed" class="region-setup__error">
        {{ store.job.error }}
      </div>

      <div v-if="isDone" class="region-setup__hint">
        The area is ready. It is now available when you add a data layer.
      </div>

      <div class="region-setup__actions">
        <button v-if="isTerminal" class="button--fit" @click="restart()">
          Load another area
        </button>
        <button class="button--fit button--accent" @click="close">
          {{ isTerminal ? "Done" : "Close and keep running" }}
        </button>
      </div>
    </div>
  </ModalDialog>
</template>

<script>
import MapMarkerIcon from "vue-material-design-icons/MapMarker.vue";
import ListContainer from "./ListContainer.vue";
import LoaderAnimation from "./LoaderAnimation.vue";
import ModalDialog from "./ModalDialog.vue";
import TextInput from "./TextInput.vue";
import { areaIdOf, useDataImportStore } from "../stores/dataImport";
import { useDataStore } from "../stores/data";

const PHASE_LABELS = {
  queued: "Queued",
  resolving: "Selecting data source",
  downloading: "Downloading OpenStreetMap data",
  parsing: "Reading street network",
  populating: "Estimating residences",
  clipping: "Trimming to your area",
  saving: "Saving",
  done: "Ready",
  failed: "Failed",
};

export default {
  components: {
    ModalDialog,
    ListContainer,
    LoaderAnimation,
    TextInput,
    MapMarkerIcon,
  },
  data() {
    return {
      store: useDataImportStore(),
      isShown: false,
    };
  },
  computed: {
    phaseLabel() {
      if (!this.store.job) return "";
      return PHASE_LABELS[this.store.job.phase] || this.store.job.phase;
    },
    isDone() {
      return !!this.store.job && this.store.job.phase === "done";
    },
    isFailed() {
      return !!this.store.job && this.store.job.phase === "failed";
    },
    isTerminal() {
      return this.isDone || this.isFailed;
    },
    showProgressBar() {
      return (
        !!this.store.job &&
        this.store.job.phase === "downloading" &&
        !!this.store.job.total_bytes
      );
    },
    downloadPercent() {
      if (!this.showProgressBar) return 0;
      return Math.min(
        100,
        Math.round(
          (this.store.job.downloaded_bytes / this.store.job.total_bytes) * 100
        )
      );
    },
  },
  mounted() {
    window.addEventListener("showRegionSetup", this.open);
  },
  unmounted() {
    window.removeEventListener("showRegionSetup", this.open);
    this.store.stopPolling();
  },
  methods: {
    open() {
      this.isShown = true;
      this.store.loadImportedRegions();
    },
    close() {
      this.isShown = false;
      // A finished import adds a usable area, so refresh the layer list the
      // data view renders from.
      if (this.isDone) useDataStore().loadLayers();
    },
    restart() {
      this.store.reset();
    },
    isImported(area) {
      return this.store.importedRegions.includes(areaIdOf(area));
    },
    // The backend builds names as "<prefix> <name>", which leaves a leading
    // space when an area has no prefix.
    areaName(area) {
      if (!area) return "";
      return (area.properties.name || "").trim();
    },
    formatSize(bytes) {
      if (!bytes) return "unknown";
      const mb = bytes / (1024 * 1024);
      if (mb >= 1024) return `${(mb / 1024).toFixed(1)} GB`;
      return `${Math.round(mb)} MB`;
    },
    // Mirrors the mapping used by DataSourceMapOverlay, per
    // https://wiki.openstreetmap.org/wiki/Tag:boundary%3Dadministrative
    getLevelDescription(level) {
      const levelDescriptions = [
        "Supernational",
        "Nation",
        "Subnation",
        "Region",
        "State-District",
        "County",
        "Administrative division",
        "Town",
        "Part of municipality",
        "Part of municipality",
      ];
      if (!levelDescriptions[level - 1]) return "";
      return levelDescriptions[level - 1];
    },
  },
};
</script>

<style lang="scss" scoped>
.region-setup {
  &__intro {
    margin: 0 0 $space-sm;
  }

  &__step {
    display: flex;
    flex-direction: column;
    gap: $space-sm;
  }

  &__search {
    display: flex;
    align-items: flex-end;
    gap: $space-sm;

    button {
      width: auto;
      padding: $space-ssm $space-sm;
    }
  }

  &__entry {
    display: flex;
    align-items: center;
    gap: $space-sm;
    width: auto;
    text-align: left;
    padding: $space-ssm $space-sm;
    margin: $space-ssm;
    border-radius: $br-md;
    background-color: var(--c-box);
    border: 1px solid var(--c-button-border);

    &:hover {
      border-color: var(--c-accent-two);
    }

    &__text {
      display: flex;
      flex-direction: column;
      flex-grow: 1;
    }

    &__title {
      font-size: $font-md;
    }
  }

  &__ready {
    color: var(--c-accent-two);
    font-size: $font-sm;
  }

  &__loading {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: $space-sm;
  }

  &__summary {
    display: flex;
    flex-direction: column;
    gap: $space-ssm;

    &__row {
      display: flex;
      justify-content: space-between;
      gap: $space-sm;
    }
  }

  &__progress {
    width: 100%;
    height: $space-ssm;
    border-radius: $br-md;
    background-color: var(--c-button-background--darkened-strong);
    overflow: hidden;

    &__bar {
      height: 100%;
      background-color: var(--c-accent-two);
      transition: width 0.3s ease;
    }
  }

  &__hint {
    font-size: $font-sm;
    margin: 0;
  }

  &__error {
    background-color: var(--c-box);
    border: 1px solid var(--c-accent-three);
    border-radius: $br-md;
    padding: $space-ssm $space-sm;
    margin-bottom: $space-sm;
  }

  &__actions {
    display: flex;
    justify-content: space-between;
    gap: $space-sm;
    padding-top: $space-ssm;

    button {
      width: auto;
      padding: $space-ssm $space-sm;
    }
  }
}
</style>
