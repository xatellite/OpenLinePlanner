<template>
  <div class="line-element">
    <div class="line-element__data">
      <button @click="openColorPick"><IconLine :color="line.color" /></button>
      <ColorPicker v-if="selectColor == true" :initColor="line.color" :handleColorChange="updateColor" :closeAction="() => selectColor = false"/>
      <input
        v-if="editStore.isEditing == line"
        type="text"
        class="line-element__name-input grow"
        :value="line.name"
        @change="editName"
      />
      <span v-else class="grow">{{ line.name }}</span>
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
    <div class="line-element__warning" v-if="editStore.isEditing === line && editStore.isExtending != null">
      <span class="line-element__warning__title">{{line.name}} is being extended!</span>
      <span>To end the line stop editing (Enter) <ArrowDownRightIcon /></span>
    </div>
  </div>
</template>

<script>
// import BusStopIcon from "vue-material-design-icons/BusStop.vue";
import PencilOutlineIcon from "vue-material-design-icons/PencilOutline.vue";
import TrashCanOutlineIcon from "vue-material-design-icons/TrashCanOutline.vue";
import ArrowDownRightIcon from "vue-material-design-icons/ArrowDownRight.vue";
import IconLine from "./icons/IconLine.vue";
import { useLinesStore } from "../stores/lines";
import { useEditStore } from "../stores/editing";
import ColorPicker from "./ColorPicker.vue";

export default {
  props: {
    line: Object,
  },
  components: {
    IconLine,
    // BusStopIcon,
    PencilOutlineIcon,
    TrashCanOutlineIcon,
    ColorPicker,
    ArrowDownRightIcon,
  },
  data() {
    return {
      linesStore: useLinesStore(),
      editStore: useEditStore(),
      selectColor: false,
    };
  },
  methods: {
    editName(e) {
      this.linesStore.getLineById(this.line.id).name = e.srcElement.value;
    },
    updateColor(color) {
      this.linesStore.updateLineColor(this.line.id, color);
    },
    toggleEditing() {
      this.editStore.isEditing = this.line;
      if (this.line.pointIds.length === 0) {
        this.editStore.isExtending = -1;
        return;
      }
      this.editStore.isExtending = null;
    },
    findNewStation() {},
    openColorPick(e) {
      e.stopPropagation();
      this.selectColor = true;
    },
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
  flex-direction: column;

  &__data {
    display: flex;
    align-items: center;
    padding: $space-ssm $space-ssm;
  }

  &__warning {
    display: flex;
    flex-direction: column;
    align-items: center;
    background-color: $c-accent-one;
    // color: $c-text-pr;
    padding: $space-ssm $space-ssm;

    &__title {
      font-weight: 700;
    }

    svg {
      height: 1rem;
    }
  }

  &__name-input {
    border: none;
    background-color: transparent;
    text-decoration: underline;
    font-size: $font-md;
    width: 0;
  }

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
