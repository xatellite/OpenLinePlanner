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
import { useLinesStore } from "@/stores/lines";
import { usePaxStore } from "@/stores/pax";
import { useEditStore } from "@/stores/editing";
import { getStreetAddressName } from "@/helpers/api";

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
      const updatedPoint = this.point.copy();
      // Fetch street address name if possible
      getStreetAddressName(this.point)
        .then((name) => {
          updatedPoint.name = name;
        })
        .finally(() => {
          updatedPoint.type = "station";
          this.linesStore.points[this.point.id] = updatedPoint;
          this.paxStore.setCurrent(false);
        });
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
  gap: $space-ssm;
}

.trash {
  color: red;
}
</style>
