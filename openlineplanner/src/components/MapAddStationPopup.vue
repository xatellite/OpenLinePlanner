<template>
  <div class="box add-station">
    <TooltipButton v-if="isLast()" :handler="extendLine" toolTip="Extend line">
      <PlusIcon />
    </TooltipButton>
    <button @click="addStation"><BusStopIcon /></button>
    <button class="trash" @click="removePoint"><TrashCanOutlineIcon /></button>
  </div>
</template>

<script>
import PlusIcon from "vue-material-design-icons/Plus.vue";
import BusStopIcon from "vue-material-design-icons/BusStop.vue";
import TrashCanOutlineIcon from "vue-material-design-icons/TrashCanOutline.vue";
import TooltipButton from "./TooltipButton.vue";
import { useLinesStore } from "../stores/lines";
import { usePaxStore } from "../stores/pax";
import { useEditStore } from "../stores/editing";

export default {
  props: {
    point: Object,
  },
  components: {
    PlusIcon,
    TrashCanOutlineIcon,
    BusStopIcon,
    TooltipButton,
  },
  data() {
    return {
      linesStore: useLinesStore(),
      editStore: useEditStore(),
      paxStore: usePaxStore(),
      lineExtendIndex: -1,
    };
  },
  methods: {
    addStation() {
      this.setStreetAddressName(this.point);
      const updatedPoint = this.point.copy();
      updatedPoint.type = "station";
      this.linesStore.points[this.point.id] = updatedPoint;
      this.paxStore.setCurrent(false);
    },
    // Duplicate with ActionToolbar
    setStreetAddressName(point) {
      fetch("https://nominatim.openstreetmap.org/reverse.php?lat="+point.lat+"&lon="+point.lng+"&zoom=18&format=jsonv2", {
          method: "GET"
        })
          .then((data) => data.json())
          .then((geocodingResult) => {
            const updatedPoint = this.linesStore.points[point.id];
            updatedPoint.name = geocodingResult.address.road;
            this.linesStore.points[point.id] = updatedPoint;
          })
    },
    removePoint() {
      // Matomo Tracking
      if (window && window.Piwik) {
        window.Piwik.getTracker().trackEvent("editing", "removePoint");
      }
      this.linesStore.removePoint(this.point.id);
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
  },
};
</script>

<style lang="scss" scoped>
.add-station {
  position: absolute;
  top: auto;
  bottom: $space-md;
  display: flex;
  padding: $space-sm;
  font-size: $font-md;
  height: fit-content;
  align-items: center;
  justify-content: center;
}

.trash {
  color: red;
}
</style>
