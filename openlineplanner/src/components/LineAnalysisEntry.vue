<template>
  <div class="line-analysis">
    <span class="line-analysis__title">
      {{ line.name }} - <TypeIcon :type="line.type" /> /
      <IconLine :color="line.color" />
    </span>
    <div
      :class="
        minLineLength > 100
          ? 'line-analysis__box  line-analysis__box--overflow'
          : 'line-analysis__box'
      "
    >
      <div
        class="line-analysis__line"
        :style="`border-color: ${line.color}; width: ${minLineLength}%`"
      >
        <div
          v-for="(station, index) in calculateStationData()"
          :key="`${index}-station`"
          class="line-analysis__stop station"
          :style="`left: ${station.x}%`"
        >
          <div class="line-analysis__stop__info">
            <b>{{ station.name ? station.name : "unnamed Station" }}</b>
            <span>{{ station.driveTime }}min</span>
            <span>{{ station.distance }}m</span>
          </div>
        </div>
      </div>
    </div>
    <div class="line-analysis__totals">
      <span>Total distance: <b>{{ totalDistance }}m</b></span>
      <span>Total time: <b>{{ totalTravelTime }}</b></span>
    </div>
  </div>
</template>

<script>
// ToDo: Add manual perspective adjust
import TransportLine from "../helpers/classes/TransportLine";
import TypeIcon from "./TypeIcon.vue";
import IconLine from "./IconLine.vue";
import { useLinesStore } from "../stores/lines";
import { calculateMidPoint } from "../helpers/geometry";
import { secondsToMinSecPadded } from "../helpers/time";

export default {
  props: {
    line: TransportLine,
  },
  data() {
    return {
      lineStore: useLinesStore(),
      minLineLength: 100,
      totalDistance: 0,
      totalTravelTime: 0,
    };
  },
  methods: {
    calculateStationData() {
      let lastPoint = null;
      const distances = [];
      const points = [];
      this.line.pointIds.forEach((pointId, index) => {
        const point = this.lineStore.getPointById(pointId);
        points.push(point);
        if (index > 0) {
          distances.push(calculateMidPoint(lastPoint, point).totalDistance);
        }
        lastPoint = point;
      });
      this.totalDistance = distances.reduce((a, b) => a + b);
      const stations = [];
      let totalDistanceCounter = 0;
      let distanceCounter = 0;
      let absoluteXOffset = 0;
      let totalTravelTimeSeconds = 0;
      let lastOffset = -100;
      points.forEach((point, index) => {
        if (index > 0) {
          totalDistanceCounter += distances[index - 1];
          distanceCounter += distances[index - 1];
        }
        let xOffset = Math.round(
          (totalDistanceCounter / this.totalDistance) * 100
        );
        if (xOffset - lastOffset < 10) {
          absoluteXOffset += 20 - (xOffset - lastOffset);
        }
        lastOffset = xOffset;

        if (point.type === "station") {
          const maxSpeed = this.getMaxSpeedForSegment(points, index);
          const driveTime = this.calculateDriveTime(distanceCounter, maxSpeed);
          totalTravelTimeSeconds += driveTime;
          stations.push({
            ...point,
            distance: totalDistanceCounter,
            x: xOffset,
            driveTime: secondsToMinSecPadded(driveTime),
          });
          distanceCounter = 0;
        }
      });
      this.minLineLength = 100 + absoluteXOffset;
      this.totalTravelTime = secondsToMinSecPadded(totalTravelTimeSeconds);
      return stations;
    },
    getMaxSpeedForSegment(points, index) {
      if (index <= 0) {
        return this.line.getMaxSpeed();
      }
      const previousPoint = points[index - 1];
      if (this.line.customSpeedLimits[previousPoint.id]) {
        return this.line.customSpeedLimits[previousPoint.id];
      }
      return this.line.getMaxSpeed();
    },
    // Calculate travel time based on acceleration data
    calculateDriveTime(distance, maxSpeed) {
      const a = this.line.getAcceleration();
      const vMax = maxSpeed / 3.6;
      const aDistance = vMax / a;
      let legTime = 0;
      // Handle acceleration distance to short
      if (distance < 2 * aDistance) {
        const legVMax = Math.sqrt((distance / 2) * (a ^ 2));
        legTime = legVMax / a;
      } else {
        const leftDistance = distance - 2 * aDistance;
        const steadyTime = leftDistance / vMax;
        const aTime = vMax / a;
        legTime = aTime * 2 + steadyTime;
      }
      legTime += this.line.getStopTime();
      return legTime;
    },
  },
  components: { TypeIcon, IconLine },
};
</script>

<style lang="scss">
.line-analysis {
  margin: $space-md $space-sm;
  border-radius: $br-md;
  border: 1px solid rgba($c-text-primary, 0.2);

  &__title {
    padding: $space-sm $space-sm 0;
    display: flex;
    align-items: center;
  }

  &__box {
    padding: 0 $space-sm; 
    position: relative;
    padding-right: 100px;

    &--overflow {
      overflow-x: auto;
      overflow-y: hidden;
    }
  }

  &__line {
    position: relative;
    width: 100%;
    border-bottom: 3px solid;
    border-radius: 3px;
    margin: $space-md 0 100px;
    box-sizing: border-box;
  }

  &__stop {
    position: absolute;
    top: -8px;

    &__info {
      display: flex;
      flex-direction: column;
      margin: $space-md 0;
      margin-left: 7px;
      border-left: 1px solid rgba($c-text-primary, 0.2);
      padding-left: $space-sm;
      width: 100px;
    }
  }

  &__totals {
    border-top: 1px solid rgba($c-text-primary, 0.2);
    padding: $space-sm;
    display: flex;
    justify-content: space-evenly;
  }
}
</style>
