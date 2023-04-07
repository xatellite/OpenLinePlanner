<template>
  <div class="chart">
    <div class="chart__container">
      <apexchart
        width="300"
        type="bar"
        :options="optionsTransport"
        :series="seriesTransport"
      />
    </div>
    <div class="chart__container">
      <apexchart
        width="300"
        type="bar"
        :options="optionsType"
        :series="seriesType"
      />
    </div>
  </div>
</template>

<script>
export default {
  props: {
    seriesTransport: Array,
    seriesType: Array,
  },
  data() {
    return {
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
    };
  },
}
</script>

<style lang="scss" scoped>
.chart {
  display: flex;

  &__container {
    font-size: $font-md;
    border: 1px solid var(--c-primary-light);
    border-radius: $br-md;
    margin: $space-ssm;
  }
}
</style>
