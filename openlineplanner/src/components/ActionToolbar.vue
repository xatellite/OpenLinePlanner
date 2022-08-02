<template>
  <div class="action-toolbar">
    <div class="action-toolbar__standard">
      <button @click="save"><TrayArrowDownIcon /></button>
      <button @click="load"><FolderUploadOutlineIcon /></button>
      <button @click="generatePdf"><FileExportOutlineIcon /></button>
    </div>
    <button
      class="action-toolbar__stop"
      v-if="editStore.isEditing"
      @click="disableEditing"
    >
      <PencilOffOutlineIcon />
    </button>
  </div>
</template>

<script>
import TrayArrowDownIcon from "vue-material-design-icons/TrayArrowDown.vue";
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

export default {
  components: {
    TrayArrowDownIcon,
    FolderUploadOutlineIcon,
    PencilOffOutlineIcon,
    FileExportOutlineIcon,
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
      if (e.key === "Enter") {
        this.disableEditing();
      }
    });
  },
  methods: {
    disableEditing() {
      this.editStore.isEditing = null;
      this.editStore.isExtending = null;
    },
    save() {
      const linesStore = this.linesStore.$state;
      downloadJSON({ linesStore });
    },
    load() {
      selectFile((file) => {
        readJSONFile(file, (json) => {
          this.editStore.stopAllInputs();
          this.linesStore.loadState(json.linesStore);
        });
      });
    },
    generatePdf() {
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
            Object.values(stationsData).map(
              async (stationData, stationIndex) => {
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
                  colors: [
                    "#424242",
                    "#47AEE9",
                    "#EEC83F",
                    "#54BA7D",
                    "#BA546C",
                  ],
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

                let stationName = this.linesStore.getPointById(
                  stationData.id
                ).name;
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
              }
            )
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
