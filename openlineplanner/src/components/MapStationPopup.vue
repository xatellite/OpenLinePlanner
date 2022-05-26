<template>
  <div class="box station-popup">
    <div class="station-popup__title-container">
      <span class="station-popup__title">Station</span>
      <div class="station-popup__actions">
        <button v-if="isLast" @click=""><PlusIcon /></button>
        <button class="trash" @click="removePoint"><TrashCanOutlineIcon /></button>
      </div>
    </div>
    <div class="charts" v-if="seriesTransport.length > 0 && paxStore.isCurrent">
      <div class="chart-container">
        <apexchart width="300" type="bar" :options="optionsTransport" :series="seriesTransport" />
      </div>
      <div class="chart-container">
        <apexchart width="300" type="bar" :options="optionsType" :series="seriesType" />
      </div>
    </div>
    <span v-else> loading.. </span>

  </div>
</template>

<script>
import { useLinesStore } from "../stores/lines";
import { usePaxStore } from "../stores/pax";
import PlusIcon from "vue-material-design-icons/Plus.vue";
import TrashCanOutlineIcon from "vue-material-design-icons/TrashCanOutline.vue";
import { useEditStore } from '../stores/editing';

export default {
  props: {
    point: Object,
  },
  components: {
    PlusIcon,
    TrashCanOutlineIcon,
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
        colors: ['#424242', '#5DE947', '#54BA7D', '#BA546C'],
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
        colors: ['#424242', '#47AEE9', '#EEC83F', '#54BA7D', '#BA546C'],
        plotOptions: {
          bar: {
            horizontal: true,
            distributed: true,
          },
        },
      },
      seriesType: [],
      seriesTransport: [],
      isLast: false,
    };
  },
  mounted() {
    this.loadInformation();

    this.paxStore.$subscribe((_, state) => {
      if (state.isCurrent == false) {
        this.loadInformation();
      }
    });
  },
  methods: {
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
    flex-grow: 1;
  }

  &__title-container {
    display: flex;
    justify-content: space-between;
    width: 100%;
  }

  &__actions {
    display: flex;
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
