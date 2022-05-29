<template>
  <div class="line-element">
    <button><IconLine :color="line.color" /></button>
    <span class="grow">{{ line.name }}</span>
    <!-- <button v-if="editStore.isEditing == line"><BusStopIcon /></button> -->
    <button
      v-if="editStore.isEditing != line"
      class="line-element__edit"
      @click="toggleEditing"
    >
      <PencilOutlineIcon />
    </button>
    <button
      v-if="editStore.isEditing == line"
      class="line-element__remove"
      @click="removeLine"
    >
      <TrashCanOutlineIcon />
    </button>
  </div>
</template>

<script>
import BusStopIcon from "vue-material-design-icons/BusStop.vue";
import PencilOutlineIcon from "vue-material-design-icons/PencilOutline.vue";
import TrashCanOutlineIcon from "vue-material-design-icons/TrashCanOutline.vue";
import IconLine from "./icons/IconLine.vue";
import { useLinesStore } from "../stores/lines";
import { useEditStore } from "../stores/editing";

export default {
  props: {
    line: Object,
  },
  components: {
    IconLine,
    BusStopIcon,
    PencilOutlineIcon,
    TrashCanOutlineIcon,
  },
  data() {
    return {
      linesStore: useLinesStore(),
      editStore: useEditStore(),
    };
  },
  methods: {
    toggleEditing() {
      this.editStore.isEditing = this.line;
      if (this.line.pointIds.length === 0) {
        this.editStore.isExtending = -1;
        return;
      }
      this.editStore.isExtending = null;
    },
    findNewStation() {},
    removeLine() {
      this.editStore.isEditing = null;
      this.editStore.isExtending = null;
      this.linesStore.removeLine(this.line);
    },
  },
};
</script>

<style lang="scss">
@import "@/assets/variables.scss";
.line-element {
  display: flex;
  align-items: center;
  padding: $space-ssm $space-sm;

  .grow {
    flex-grow: 1;
    padding: $space-ssm;
  }

  &__remove {
    color: red;
  }

  &__edit {
    display: none;
  }

  &:hover {
    .line-element__edit {
      display: block;
    }
  }
}
</style>
