<template>
  <div class="box station-popup">
    <div class="station-popup__title-container">
      <input
        type="text"
        class="station-popup__title"
        :value="point.name || 'Station'"
        @change="editName"
        @mousedown="handleMouseDown"
      />
      <div class="station-popup__actions">
        <span v-if="editStore.isMerging"
          >Merging active! Select other station to merge</span
        >
        <TooltipButton
          :handler="toggleMerge"
          toolTip="Activate/Deactivate station merging"
        >
          <TransitConnectionHorizontalIcon />
        </TooltipButton>
        <TooltipButton
          v-if="isLast()"
          :handler="extendLine"
          toolTip="Extend line"
        >
          <PlusIcon />
        </TooltipButton>
        <button class="remove" @click="removePoint">
          <TrashCanOutlineIcon />
        </button>
      </div>
    </div>
    <div class="station-popup__container">
      <div class="station-popup__chart" v-if="chartSeries.length > 0 && paxStore.isCurrent">
        <apexchart
          width="300"
          type="bar"
          :options="chartOptions"
          :series="chartSeries"
        />
      </div>
      
      <div v-else class="station-popup__lazy">
        <div class="station-popup__chart">
          <span> loading... </span>
          <div class="station-popup__lazy__chart" />
        </div>
      </div>
    </div>
  </div>
</template>

<script>
// ToDo Merge and split again with MapAddStationPopup
import { useLinesStore } from "../stores/lines";
import { usePaxStore } from "../stores/pax";
import PlusIcon from "vue-material-design-icons/Plus.vue";
import TrashCanOutlineIcon from "vue-material-design-icons/TrashCanOutline.vue";
import TransitConnectionHorizontalIcon from "vue-material-design-icons/TransitConnectionHorizontal.vue";
import { useEditStore } from "../stores/editing";
import TooltipButton from "./TooltipButton.vue";

export default {
  props: {
    point: Object,
  },
  components: {
    PlusIcon,
    TrashCanOutlineIcon,
    TransitConnectionHorizontalIcon,
    TooltipButton,
  },
  data() {
    return {
      paxStore: usePaxStore(),
      linesStore: useLinesStore(),
      editStore: useEditStore(),
      chartSeries: [],
      chartOptions: [],
      lineExtendIndex: -1,
    };
  },
  mounted() {
    this.loadInformation();

    this.paxStore.$onAction(({ name, after }) => {
      if (name === "setCurrent") {
        after((isCurrent) => {
          if (isCurrent == false) {
            this.loadInformation();
          }
        });
      }
    });
  },
  methods: {
    handleMouseDown(e) {
      e.stopPropagation();
    },
    editName(e) {
      this.linesStore.getPointById(this.point.id).name = e.srcElement.value;
    },
    toggleMerge(e) {
      e.stopPropagation();
      // Matomo Tracking
      if (window && window.Piwik) {
        window.Piwik.getTracker().trackEvent("editing", "toggleMerge");
      }
      if (this.editStore.isMerging) {
        this.editStore.isMerging = null;
        return;
      }
      this.editStore.isMerging = this.point;
    },
    loadInformation() {
      this.paxStore
        .getPaxForStation(this.point.id, this.linesStore)
        .then((stationData) => {
          // ToDo: Merge series build with chart series build
          this.chartSeries = [
            {
              name: "series-1",
              data: [
                stationData.reduce((partialSum, entry) => partialSum + entry.value, 0),
                ...stationData.map((entry) => entry.value),
              ],
            },
          ];
          this.chartOptions = {
            chart: {
              id: "station-chart",
            },
            title: {text:'Catchment Area'},
            xaxis: {
              categories: [ "Total", ...stationData.map((entry) => entry.layer_type)],
            },
            colors: ["#424242", "#5DE947", "#54BA7D", "#BA546C"], //ToDo: Add colors to Endpoint?
            plotOptions: {
              bar: {
                horizontal: true,
                distributed: true,
              },
            },
          };
        });
    },
    extendLine(e) {
      e.stopPropagation();
      // Matomo Tracking
      if (window && window.Piwik) {
        window.Piwik.getTracker().trackEvent("editing", "extendLine");
      }
      this.editStore.isEditing = this.linesStore.getLineById(this.isLast());
      this.editStore.isExtending = this.lineExtendIndex;
      this.editStore.pointSelected = null;
    },
    isLast() {
      let lineEndPointRef = false;
      this.point.lines.forEach((lineRef) => {
        const linePoints = this.linesStore.getLineById(lineRef).pointIds;
        if (linePoints[0] === this.point.id) {
          lineEndPointRef = lineRef;
          this.lineExtendIndex = 0;
        } else if (linePoints[linePoints.length - 1] === this.point.id) {
          lineEndPointRef = lineRef;
          this.lineExtendIndex = -1;
        }
      });
      return lineEndPointRef;
    },
    removePoint() {
      // Matomo Tracking
      if (window && window.Piwik) {
        window.Piwik.getTracker().trackEvent("editing", "removePoint");
      }
      this.linesStore.removePoint(this.point.id);
    },
  },
};
</script>

<style lang="scss" scoped>
.station-popup {
  position: absolute;
  top: auto;
  bottom: $space-md;
  display: flex;
  // align-items: center;
  justify-content: center;
  flex-direction: column;
  z-index: 4;
  overflow: hidden;
  border: 1px solid var(--c-button-border);

  &__title {
    font-size: $font-lg;
    padding: $space-ssm;
    font-weight: 700;
    padding: auto 0;
    pointer-events: auto;
    border: 0;
    background-color: transparent;
  }

  &__title-container {
    display: flex;
    justify-content: space-between;
    width: 100%;
    padding: $space-ssm;
    box-sizing: border-box;
    border-bottom: 1px solid var(--c-button-border);
  }

  &__actions {
    display: flex;
    align-items: center;
    gap: $space-ssm;
  }

  &__lazy {
    position: relative;
    display: flex;

    &__chart {
      position: absolute;
      z-index: -1;
      width: 280px;
      height: 190px;
      padding: 5px 10px;
      background-image: url(@/assets/lazy_graph.png);
      // filter: blur($space-sm);
      animation: infinite-blur-change 1s linear infinite;
    }
  }

  &__container {
    background-color: var(--c-button-background);
    padding: $space-ssm;
  }


  &__chart {
      position: relative;
      display: flex;
      align-items: center;
      font-size: $font-md;
      z-index: 10;
      justify-content: center;
      border: 1px solid var(--c-button-border);
      border-radius: $br-md;
      padding: $space-ssm;
      margin: $space-ssm;
      background-color: var(--c-box);
      width: 300px;
      height: 200px;
    }
}
</style>
