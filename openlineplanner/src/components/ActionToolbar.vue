<template>
  <div class="action-toolbar">
    <div class="action-toolbar__standard">
      <TooltipButton :handler="save" toolTip="Save configuration as file"
        ><TrayArrowDownIcon
      /></TooltipButton>
      <TooltipButton :handler="load" toolTip="Load configuration from file"
        ><FolderUploadOutlineIcon
      /></TooltipButton>
      <TooltipButton
        :handler="generatePdf"
        toolTip="Export configuration to PDF"
        ><FileExportOutlineIcon
      /></TooltipButton>
    </div>
    <TooltipButton toolTip="Check Line Infos" href="/analysis">
      <BlurLinearIcon />
    </TooltipButton>
    <TooltipButton
      v-if="editStore.isEditing"
      :handler="disableEditing"
      toolTip="Stop editing"
    >
      <span class="action-toolbar__stop"><PencilOffOutlineIcon /></span>
    </TooltipButton>
  </div>
</template>

<script>
import TrayArrowDownIcon from "vue-material-design-icons/TrayArrowDown.vue";
import BlurLinearIcon from "vue-material-design-icons/BlurLinear.vue";
import FolderUploadOutlineIcon from "vue-material-design-icons/FolderUploadOutline.vue";
import PencilOffOutlineIcon from "vue-material-design-icons/PencilOffOutline.vue";
import FileExportOutlineIcon from "vue-material-design-icons/FileExportOutline.vue";
import { jsPDF } from "jspdf";
import { useEditStore } from "../stores/editing";
import { useLinesStore } from "../stores/lines";
import { selectFile, downloadJSON, readJSONFile } from "../helpers/file";
import { usePaxStore } from "../stores/pax";
import ApexCharts from "apexcharts";
import { useOverlayStore } from "../stores/overlay";
import TooltipButton from "./TooltipButton.vue";

export default {
  components: {
    TrayArrowDownIcon,
    FolderUploadOutlineIcon,
    PencilOffOutlineIcon,
    FileExportOutlineIcon,
    TooltipButton,
    BlurLinearIcon,
  },
  setup() {
    const editStore = useEditStore();
    const linesStore = useLinesStore();
    const paxStore = usePaxStore();
    const overlayStore = useOverlayStore();

    return {
      editStore,
      linesStore,
      paxStore,
      overlayStore,
    };
  },
  mounted() {
    window.addEventListener("keydown", (e) => {
      if (this.editStore.isExtending) {
        if (e.key === "Enter") {
          this.disableEditing();
        }
      }
    });
  },
  methods: {
    disableEditing() {
      this.editStore.isEditing = null;
      this.editStore.isExtending = null;
    },
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
      const linesStore = this.linesStore.$state;
      downloadJSON({ linesStore });
    },
    load() {
      selectFile((file) => {
        readJSONFile(file, (json) => {
          this.editStore.stopAllInputs();
          this.linesStore.loadState(json.linesStore);
          // Matomo tracking
          if (window && window.Piwik) {
            window.Piwik.getTracker().trackEvent("editing", "load", {
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
    generatePdf() {
      // Matomo tracking
      if (window && window.Piwik) {
        window.Piwik.getTracker().trackEvent("editing", "pdf", {
          lines: Object.keys(this.linesStore.lines).length,
          points: Object.keys(this.linesStore.points).length,
          stations: Object.values(this.linesStore.points)
            .map((point) => point.type === "station")
            .reduce((a, b) => a + b),
        });
      }
      document.addEventListener(
        "mapExportGenerated",
        async (exportedImageEvent) => {
          const doc = new jsPDF({
            orientation: "landscape",
          });
          doc.addImage({
            imageData: exportedImageEvent.detail.urlData,
            x: 0,
            y: 0,
            width: 297,
            height: exportedImageEvent.detail.heightRatio * 297,
          });
          doc.insertPage();
          const stationsData = await this.paxStore.getPaxForAllStations(
            this.linesStore
          );

          if (!stationsData) {
            this.overlayStore.exporting = false;
          }

          doc.setFont("helvetica", "normal", 700);
          await Promise.all(
            Object.keys(stationsData).map(async (stationId, stationIndex) => {
              const stationData = stationsData[stationId];
              const seriesTransport = [
                {
                  name: "series-1",
                  data: [
                    Math.round(stationData.total),
                    Math.round(stationData.ped),
                    Math.round(stationData.bike),
                    Math.round(stationData.car),
                  ],
                },
              ];
              const seriesType = [
                {
                  name: "series-1",
                  data: [
                    Math.round(stationData.total),
                    Math.round(stationData.leisure),
                    Math.round(stationData.school),
                    Math.round(stationData.residential),
                    Math.round(stationData.work),
                  ],
                },
              ];
              const optionsTransport = {
                plotOptions: {
                  bar: {
                    horizontal: true,
                    distributed: true,
                  },
                },
                colors: ["#424242", "#5DE947", "#54BA7D", "#BA546C"],
                xaxis: {
                  categories: ["total", "foot", "bike", "car"],
                },
                chart: {
                  id: "station-chart",
                  type: "bar",
                  animations: {
                    enabled: false,
                  },
                  height: 300,
                  width: 300,
                },
                series: seriesTransport,
              };

              const optionsType = {
                chart: {
                  id: "station-chart",
                  type: "bar",
                  animations: {
                    enabled: false,
                  },
                  height: 300,
                  width: 300,
                },
                colors: ["#424242", "#47AEE9", "#EEC83F", "#54BA7D", "#BA546C"],
                plotOptions: {
                  bar: {
                    horizontal: true,
                    distributed: true,
                  },
                },
                series: seriesType,
                xaxis: {
                  categories: [
                    "total",
                    "leisure",
                    "school",
                    "residence",
                    "work",
                  ],
                },
              };

              const canvasTransportDiv = document.createElement("div");
              document.body.appendChild(canvasTransportDiv);
              const transportChart = new ApexCharts(
                canvasTransportDiv,
                optionsTransport
              );
              await transportChart.render();
              const transportChartImage = await transportChart.dataURI();
              document.body.removeChild(canvasTransportDiv);

              const canvasTypeDiv = document.createElement("div");
              document.body.appendChild(canvasTypeDiv);
              const typeChart = new ApexCharts(canvasTypeDiv, optionsType);
              await typeChart.render();
              const typeChartImage = await typeChart.dataURI();
              document.body.removeChild(canvasTypeDiv);

              let stationName = this.linesStore.getPointById(stationId).name;
              if (stationName.length <= 0) {
                stationName = `Station ${stationIndex + 1}`;
              }
              if (stationIndex >= 4 && stationIndex % 4 === 0) {
                doc.insertPage();
              }
              const xOffset = 25 + (stationIndex % 2) * 148;
              const yOffset = 25 + Math.floor((stationIndex % 4) / 2) * 105;

              doc.text(stationName, xOffset, yOffset);
              doc.addImage({
                imageData: transportChartImage.imgURI,
                x: xOffset,
                y: yOffset + 4,
                width: 52,
                height: 52,
              });

              doc.addImage({
                imageData: typeChartImage.imgURI,
                x: xOffset + 52,
                y: yOffset + 4,
                width: 52,
                height: 52,
              });
            })
          );
          doc.save("export.pdf");
          this.overlayStore.exporting = false;
        }
      );
      this.overlayStore.exporting = true;
      const event = new CustomEvent("generateMapExport", {
        bubbles: true,
      });
      this.$el.dispatchEvent(event);
    },
  },
};
</script>

<style lang="scss">
.action-toolbar {
  display: flex;
  align-items: center;
  padding: $space-ssm;
  justify-content: space-between;
  // background-color: $c-text-primary;

  * > {
    margin: $space-ssm;
  }

  &__standard {
    display: flex;
  }

  &__stop {
    color: red;
  }
}
</style>
