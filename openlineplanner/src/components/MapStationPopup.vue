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
        <span v-if="editStore.isMerging">Merging active! Select other station to merge</span>
        <button @click="toggleMerge"><TransitConnectionHorizontalIcon /></button>
        <button v-if="isLast()" @click="extendLine"><PlusIcon /></button>
        <button class="trash" @click="removePoint">
          <TrashCanOutlineIcon />
        </button>
      </div>
    </div>
    <div class="charts" v-if="seriesTransport.length > 0 && paxStore.isCurrent">
      <div class="chart-container">
        <apexchart
          width="300"
          type="bar"
          :options="optionsTransport"
          :series="seriesTransport"
        />
      </div>
      <div class="chart-container">
        <apexchart
          width="300"
          type="bar"
          :options="optionsType"
          :series="seriesType"
        />
      </div>
    </div>
    <span v-else> loading.. </span>
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

export default {
  props: {
    point: Object,
  },
  components: {
    PlusIcon,
    TrashCanOutlineIcon,
    TransitConnectionHorizontalIcon,
  },
  data() {
    return {
      paxStore: usePaxStore(),
      linesStore: useLinesStore(),
      editStore: useEditStore(),
      optionsTransport: {
        chart: {
          id: "station-chart",
        },
        xaxis: {
          categories: ["total", "foot", "bike", "car"],
        },
        colors: ["#424242", "#5DE947", "#54BA7D", "#BA546C"],
        plotOptions: {
          bar: {
            horizontal: true,
            distributed: true,
          },
        },
      },
      optionsType: {
        chart: {
          id: "station-chart",
        },
        xaxis: {
          categories: ["total", "leisure", "school", "residence", "work"],
        },
        colors: ["#424242", "#47AEE9", "#EEC83F", "#54BA7D", "#BA546C"],
        plotOptions: {
          bar: {
            horizontal: true,
            distributed: true,
          },
        },
      },
      seriesType: [],
      seriesTransport: [],
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
          this.seriesTransport = [
            {
              name: "series-1",
              data: [
                stationData.total,
                stationData.ped,
                stationData.bike,
                stationData.car,
              ],
            },
          ];
          this.seriesType = [
            {
              name: "series-1",
              data: [
                stationData.total,
                stationData.leisure,
                stationData.school,
                stationData.residential,
                stationData.work,
              ],
            },
          ];
        });
    },
    extendLine(e) {
      e.stopPropagation();
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
      this.linesStore.removePoint(this.point.id);
    },
  },
};
</script>

<style lang="scss" scoped>
@import "@/assets/variables.scss";
.station-popup {
  position: absolute;
  top: auto;
  bottom: $space-md;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-direction: column;
  padding: $space-ssm;

  &__title {
    font-size: $font-lg;
    padding: $space-ssm;
    font-weight: bold;
    padding: auto 0;
    pointer-events: auto;
    border: 0;
    background-color: transparent;
  }

  &__title-container {
    display: flex;
    justify-content: space-between;
    width: 100%;
  }

  &__actions {
    display: flex;
    align-items: center;
  }
}

.trash {
  color: red;
}
.charts {
  display: flex;
}
.chart-container {
  font-size: $font-md;
  border: 1px solid $c-primary-light;
  border-radius: $br-md;
  margin: $space-ssm;
}
</style>
