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
      <span>Total time: <b>{{ totalTravelTime }} (incl. stop times)</b></span>
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
      let segments = [];
      const linePoints = [];
      // Collect segments
      this.line.pointIds.forEach((pointId, index) => {
        const point = this.lineStore.getPointById(pointId);
        linePoints.push(point);
        if (index > 0) {
          segments.push({
            distance: calculateMidPoint(lastPoint, point).totalDistance,
            speed: this.getMaxSpeedForSegment(this.line.pointIds, index) / 3.6,
            endsAtStation: point.type === "station",
          });
        }
        lastPoint = point;
      });
      // Calculate drive time
      segments = this.calculateSegmentTime(segments);
      this.totalDistance = segments.reduce((a, b) => a + b.distance, 0);
      const totalTravelTime = segments.reduce((a, b) => a + b.time, 0);

      const stations = [];
      let totalDistanceCounter = 0;
      let sectorTravelTime = 0;
      let timeAtStations = 0;
      linePoints.forEach((point, index) => {
        if (index > 0) {
          const segmentDistance = segments[index - 1].distance;
          totalDistanceCounter += segmentDistance;
          sectorTravelTime += segments[index - 1].time;
        }
        if (point.type === "station") {
          stations.push({
            ...point,
            distance: totalDistanceCounter,
            x: (totalDistanceCounter / this.totalDistance) * 100,
            driveTime: secondsToMinSecPadded(sectorTravelTime),
          });
          sectorTravelTime = 0;
          timeAtStations += this.line.getStopTime();
        }
      });
      this.minLineLength = 100;
      this.totalTravelTime = secondsToMinSecPadded(totalTravelTime + timeAtStations);
      return stations;
    },
    getMaxSpeedForSegment(points, index) {
      if (index <= 0) {
        return this.line.getMaxSpeed();
      }
      const previousPoint = points[index - 1];
      if (this.line.customSpeedLimits[previousPoint]) {
        return this.line.customSpeedLimits[previousPoint];
      }
      return this.line.getMaxSpeed();
    },
    calculateSegmentTime(segments) {
      const a = this.line.getAcceleration();
      // Prepare segment speeds
      segments.forEach((segment, index) => {
        if (index === 0) {
          segment.startSpeed = 0;
        } else {
          segment.startSpeed = segments[index - 1].endSpeed;
        }
        if (index >= segments.length - 1 || segment.endsAtStation) {
          segment.endSpeed = 0;
        } else {
          const requiredTime = (segment.speed - segment.startSpeed) / a;
          const requiredDistance = (segment.startSpeed * requiredTime) + 0.5 * (segment.speed - segment.startSpeed) * requiredTime;
          if (requiredDistance > segment.distance) {
            // Calculate max speed reachable in segment
            segment.speed = Math.sqrt((2*segment.distance * a) + segment.startSpeed**2);
          }
          // Reduced max Speed if next sector requires lower speed limit
          segment.endSpeed = segment.speed > segments[index + 1].speed ? segments[index + 1].speed : segment.speed;
        }
      });
      // Check for too short for breaking segments
      segments.reverse();
      // ToDo: Investigate why direct assign doesnt work.
      const segmentsReverted = segments.map((segment, index) => {
        const speedDelta = segment.endSpeed - segment.startSpeed;
        if (speedDelta < 0) {
          // Check if breaking is possible
          const decelerationSpeed = Math.abs(speedDelta);
          const requiredTime = decelerationSpeed / a;
          const requiredDistance = (segment.startSpeed * requiredTime) - 0.5 * decelerationSpeed * requiredTime;
          if (requiredDistance > segment.distance) {
            // Calculate Maximum breakable speed
            const breakableSpeed = Math.sqrt((2*segment.distance * a) + segment.endSpeed**2);
            segment.startSpeed = breakableSpeed;
            segments[index + 1].endSpeed = breakableSpeed;
          }
        }
        return segment;
      });

      segmentsReverted.reverse();

      // Calculate segment times
      segmentsReverted.forEach((segment) => {
        segment.time = this.calculateDriveTime(segment.distance, segment.speed, segment.endSpeed, segment.startSpeed);
      });
      return segmentsReverted;
    },
    // Calculate travel time based on acceleration data
    calculateDriveTime(distance, maxSpeed, endSpeed, startSpeed) {
      const a = this.line.getAcceleration();
      const vMax = maxSpeed;
      const accelerationTime = (vMax - startSpeed) / a;
      const decelerationTime = (vMax - endSpeed) / a;
      const accelerationDistance = (startSpeed * accelerationTime) + (0.5 * a * (accelerationTime**2));
      const decelerationDistance = (vMax * decelerationTime) - (0.5 * a * (decelerationTime**2));
      let legTime = 0;
      // Handle acceleration distance to short
      if (distance < accelerationDistance + decelerationDistance) {
        const limitedAccelerationDistance = (endSpeed**2-startSpeed**2+2*a*distance)/(4*a);
        const reachedSpeed = Math.sqrt(startSpeed**2+(2*a*limitedAccelerationDistance));
        const limitedDecelerationDistance = distance - limitedAccelerationDistance;
        const limitedAccelerationTime = limitedAccelerationDistance / (startSpeed+0.5*(reachedSpeed - startSpeed));
        const limitedDecelerationTime = limitedDecelerationDistance / (reachedSpeed-0.5*(reachedSpeed - endSpeed));
        legTime = limitedAccelerationTime + limitedDecelerationTime;
      } else {
        const leftDistance = distance - (accelerationDistance + decelerationDistance);
        const steadyTime = leftDistance / vMax;
        legTime = accelerationTime + decelerationTime + steadyTime;
      }
      // legTime += this.line.getStopTime();
      return legTime;
    },
  },
  components: { TypeIcon, IconLine },
};
</script>

<style lang="scss" scoped>
.line-analysis {
  margin: $space-md $space-sm;
  border-radius: $br-md;
  border: 1px solid rgba($c-text-primary, 0.2);
  background-color: $c-box;

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
    margin: $space-md 0 140px;
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
