<template>
  <ListContainer title="Saves">
    <div
      class="save-item"
      v-for="(save, index) in savesStore.saves"
      :key="`save-${index}`"
    >
      <span class="save-item__title">{{ save.projectName }}</span>
      <div class="save-item__bottom">
        <span>{{ save.date }}</span>
        <div class="save-item__bottom__actions">
          <TooltipButton :handler="() => savesStore.load(index)"
            ><LoadIcon
          /></TooltipButton>
          <TooltipButton :handler="() => savesStore.download(index)"
            ><FolderDownloadOutlineIcon
          /></TooltipButton>
          <TooltipButton :handler="() => savesStore.remove(index)">
            <span class="remove"><TrashCanOutlineIcon /></span>
          </TooltipButton>
        </div>
      </div>
    </div>
    <div class="upload-item">
      <TooltipButton :handler="loadFile" toolTip="Load configuration from file"
        ><FolderUploadOutlineIcon
      /></TooltipButton>
      <span>Local saves</span>
    </div>
  </ListContainer>
</template>

<script>
import FolderUploadOutlineIcon from "vue-material-design-icons/FolderUploadOutline.vue";
import LoadIcon from "vue-material-design-icons/TrayArrowUp.vue";
import TrashCanOutlineIcon from "vue-material-design-icons/TrashCanOutline.vue";
import FolderDownloadOutlineIcon from "vue-material-design-icons/FolderDownloadOutline.vue";
import ListContainer from "@/components/ListContainer.vue";
import TooltipButton from "@/components/TooltipButton.vue";
import { selectFile, readJSONFile } from "../helpers/file";
import { useEditStore } from "@/stores/editing";
import { useLinesStore } from "@/stores/lines";
import { useSavesStore } from "@/stores/saves";

export default {
  components: {
    ListContainer,
    TooltipButton,
    FolderUploadOutlineIcon,
    TrashCanOutlineIcon,
    FolderDownloadOutlineIcon,
    LoadIcon,
  },
  data() {
    return {
      editStore: useEditStore(),
      linesStore: useLinesStore(),
      savesStore: useSavesStore(),
    };
  },
  methods: {
    loadFile() {
      selectFile((file) => {
        readJSONFile(file, (json) => {
          this.editStore.stopAllInputs();
          this.linesStore.loadState(json.linesStore);
          // Matomo tracking
          if (window && window.Piwik) {
            window.Piwik.getTracker().trackEvent("editing", "load-local", {
              lines: Object.keys(this.linesStore.lines).length,
              points: Object.keys(this.linesStore.points).length,
              stations: Object.values(this.linesStore.points)
                .map((point) => point.type === "station")
                .reduce((a, b) => a + b),
            });
          }
        });
      });
    },
  },
};
</script>

<style lang="scss" scoped>
.upload-item {
  background-color: $c-box;
  margin: $space-sm;
  border: 1px solid $c-button-border;
  border-radius: $br-md;
  margin-bottom: $space-ssm;
  display: flex;
  align-items: center;
  width: 350px;
  padding: $space-ssm;
  gap: $space-sm;
  // border-bottom: 1px solid rgba($c-text-primary, 0.2);
}

.save-item {
  display: flex;
  flex-direction: column;
  background-color: $c-box;
  margin: $space-sm;
  padding: $space-ssm;
  border: 1px solid $c-button-border;
  border-radius: $br-md;

  &__title {
    padding: $space-ssm $space-sm;
    border-bottom: 1px solid $c-button-border;
  }

  &__bottom {
    display: flex;
    justify-content: space-between;
    padding: $space-ssm $space-sm;
    align-items: center;

    &__actions {
      display: flex;
      gap: $space-sm;
    }
  }
}
</style>
