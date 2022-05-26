<template>
  <div class="box add-station">
    <button v-if="isLast" @click="addStation"><PlusIcon /></button>
    <button @click="addStation"><BusStopIcon /></button>
    <button class="trash" @click="removePoint"><TrashCanOutlineIcon /></button>
  </div>
</template>

<script>
import PlusIcon from "vue-material-design-icons/Plus.vue";
import BusStopIcon from "vue-material-design-icons/BusStop.vue";
import TrashCanOutlineIcon from "vue-material-design-icons/TrashCanOutline.vue";
import { useLinesStore } from "../stores/lines";
import { usePaxStore } from '../stores/pax';

export default {
  props: {
    point: Object,
  },
  components: {
    PlusIcon,
    TrashCanOutlineIcon,
    BusStopIcon,
  },
  data() {
    return {
      linesStore: useLinesStore(),
      paxStore: usePaxStore(),
      isLast: false,
    };
  },
  methods: {
    addStation() {
      const updatedPoint = this.point.copy();
      updatedPoint.type = "station";
      console.log(updatedPoint);
      this.linesStore.points[this.point.id] = updatedPoint;
      this.paxStore.isCurrent = false;
    },
    removePoint() {
      this.linesStore.removePoint(this.point.id);
    },
  },
};
</script>

<style lang="scss" scoped>
@import "@/assets/variables.scss";
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