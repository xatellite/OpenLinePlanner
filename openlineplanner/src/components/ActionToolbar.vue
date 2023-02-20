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
    <TooltipButton toolTip="Timetable" href="/analysis">
      <BlurLinearIcon />
    </TooltipButton>
    <TooltipButton
      v-if="
        (editStore.isEditing && editStore.isEditing.pointIds.length > 1) ||
        findStationLoading
      "
      :handler="findStation"
      toolTip="Automatically find ideal station"
    >
      <LoadingIcon v-if="findStationLoading" class="loader" />
      <BusStopIcon v-else />
    </TooltipButton>
  </div>
</template>

<script>
import TrayArrowDownIcon from "vue-material-design-icons/TrayArrowDown.vue";
import BlurLinearIcon from "vue-material-design-icons/BlurLinear.vue";
import FolderUploadOutlineIcon from "vue-material-design-icons/FolderUploadOutline.vue";
import FileExportOutlineIcon from "vue-material-design-icons/FileExportOutline.vue";
import BusStopIcon from "vue-material-design-icons/BusStop.vue";
import LoadingIcon from "vue-material-design-icons/Loading.vue";
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
    FileExportOutlineIcon,
    TooltipButton,
    BlurLinearIcon,
    BusStopIcon,
    LoadingIcon,
  },
  data() {
    return {
      editStore: useEditStore(),
      linesStore: useLinesStore(),
      paxStore: usePaxStore(),
      overlayStore: useOverlayStore(),
      findStationLoading: false,
    };
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
    findStation() {
      // Matomo Tracking
      if (window && window.Piwik) {
        window.Piwik.getTracker().trackEvent("editing", "findStation");
      }
      if (!this.findStationLoading) {
        this.findStationLoading = true;
        const route = [];
        const stations = [];
        const points = Object.values(this.linesStore.points);
        if (points.length <= 0) {
          return;
        }
        // ToDo: Add max coverage
        points.forEach((point) => {
          if (point.type === "station") {
            stations.push({
              location: {
                y: point.lat,
                x: point.lng,
              },
              id: point.id,
              // Add max coverage
              coverage: Math.max(
                ...point.lines.map((lineId) =>
                  this.linesStore.getLineById(lineId).getCoverage()
                )
              ),
            });
          }
        });
        const lineData = this.editStore.isEditing;

        this.linesStore
          .getLineById(lineData.id)
          .pointIds.forEach((pointRef) => {
            const point = this.linesStore.getPointById(pointRef);
            route.push({
              y: point.lat,
              x: point.lng,
            });
          });

        fetch(import.meta.env.VITE_API_ENDPOINT + "/find-station", {
          method: "POST",
          body: JSON.stringify({
            route,
            stations,
            coverage: this.editStore.isEditing.getCoverage(),
            method: this.paxStore.calculationMethod,
          }),
          headers: {
            "Content-type": "application/json",
          },
        })
          .then((data) => data.json())
          .then((stationProposal) => {
            const newPoint = this.linesStore.addPoint(
              stationProposal.location.y,
              stationProposal.location.x,
              lineData,
              stationProposal.index
            );
            newPoint.type = "station";

            this.setStreetAddressName(newPoint);
          })
          .finally(() => {
            this.findStationLoading = false;
          });
      }
    },
    // Duplicate with MapAddStationPopup
    setStreetAddressName(point) {
      fetch(
        "https://nominatim.openstreetmap.org/reverse.php?lat=" +
          point.lat +
          "&lon=" +
          point.lng +
          "&zoom=18&format=jsonv2",
        {
          method: "GET",
        }
      )
        .then((data) => data.json())
        .then((geocodingResult) => {
          const updatedPoint = this.linesStore.points[point.id];
          updatedPoint.name = geocodingResult.address.road;
          this.linesStore.points[point.id] = updatedPoint;
        });
    },
  },
};
</script>

<style lang="scss">
.action-toolbar {
  display: flex;
  align-items: center;
  padding: 0 $space-ssm;
  padding-top: $space-sm;
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
