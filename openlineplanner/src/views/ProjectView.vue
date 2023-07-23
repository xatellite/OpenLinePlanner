<template>
  <main>
    <div class="project">
      <TextInput
        title="Project name"
        :value="linesStore.projectName"
        :callback="updateProjectName"
      />
      <div>
        <!-- ToDo: To be implemented in coming feature -->
        <!-- <div class="project__item">
          <TooltipButton toolTip="Save to Pdf">
            <FileExportOutlineIcon />
          </TooltipButton>
          <span>Export to Pdf</span>
        </div>
        <div class="project__item">
          <TooltipButton toolTip="Save to Excel">
            <TableArrowRightIcon />
          </TooltipButton>
          <span>Export to Excel</span>
        </div> -->
        <div id="save-project-row" class="project__item">
          <TooltipButton :handler="save" toolTip="Save configuration">
            <TrayArrowDownIcon />
          </TooltipButton>
          <span>Save Project</span>
        </div>
      </div>
      <SavesList />
    </div>
    <InteractiveMap />
  </main>
</template>

<script>
import InteractiveMap from "@/components/InteractiveMap.vue";
import TrayArrowDownIcon from "vue-material-design-icons/TrayArrowDown.vue";
// import TableArrowRightIcon from "vue-material-design-icons/TableArrowRight.vue";
// import FileExportOutlineIcon from "vue-material-design-icons/FileExportOutline.vue";
import SavesList from "@/components/SavesList.vue";
import TextInput from "@/components/TextInput.vue";
import TooltipButton from "@/components/TooltipButton.vue";
import { useLinesStore } from "@/stores/lines";
import { useSavesStore } from "@/stores/saves";

export default {
  components: {
    // FileExportOutlineIcon,
    // TableArrowRightIcon,
    InteractiveMap,
    SavesList,
    TextInput,
    TooltipButton,
    TrayArrowDownIcon,
  },
  data() {
    return {
      linesStore: useLinesStore(),
      saveStore: useSavesStore(),
    };
  },
  methods: {
    save() {
      // Matomo tracking
      if (window && window.Piwik) {
        window.Piwik.getTracker().trackEvent("editing", "save", {
          lines: Object.keys(this.linesStore.lines).length,
          points: Object.keys(this.linesStore.points).length,
          stations: Object.values(this.linesStore.points)
            .map((point) => point.type === "station")
            .reduce((a, b) => a + b),
        });
      }
      const d = new Date();
      const date = d.toLocaleDateString();
      const linesStore = {...this.linesStore.$state, date};
      this.saveStore.save(linesStore);
    },
    updateProjectName(value) {
      this.linesStore.projectName = value;
    },
  },
};
</script>

<style lang="scss" scoped>
.project {
  display: flex;
  flex-direction: column;

  &__item {
    display: flex;
    gap: $space-sm;
    margin: $space-sm 0;
    align-items: center;
  }
}
</style>
