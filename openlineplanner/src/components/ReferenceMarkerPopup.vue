<template>
  <div
    id="ref-marker-popup"
    class="box ref-marker-popup"
    @mousedown="handleDrag"
  >
    <div class="ref-marker-popup__title">
      <span>Section max Speed: </span
      ><b
        >{{
          linesStore
            .getLineById(editStore.isEditing.id)
            .getSectionSpeed(pointOne.id)
        }}km/h</b
      >
    </div>
    <div class="ref-marker-popup__sliders">
      <VueSlider
        class="ref-marker-popup__sliders__element"
        :modelValue="
          linesStore
            .getLineById(editStore.isEditing.id)
            .getSectionSpeed(pointOne.id)
        "
        :max="200"
        tooltip="none"
        :marks="[0, 30, 50, 100, 150, 200]"
        @update:modelValue="handleMaxSpeedInput"
      />
      <TooltipButton
        v-if="
          Object.keys(
            linesStore.getLineById(editStore.isEditing.id).customSpeedLimits
          ).includes(pointOne.id)
        "
        toolTip="Reset to base speed"
        :handler="handleCancel"
      >
        <CancelIcon />
      </TooltipButton>
    </div>
  </div>
</template>

<script>
import CancelIcon from "vue-material-design-icons/Cancel.vue";
import TooltipButton from "./TooltipButton.vue";
import { useEditStore } from "../stores/editing";
import { useLinesStore } from "../stores/lines";
import VueSlider from "vue-slider-component";

export default {
  props: {
    pointOne: Object,
    closeAction: Function,
  },
  components: { TooltipButton, CancelIcon, VueSlider },
  data: function () {
    return {
      editStore: useEditStore(),
      linesStore: useLinesStore(),
    };
  },
  mounted() {
    window.addEventListener("click", this.handleClickOutside);
  },
  methods: {
    handleClickOutside(e) {
      let refElement = e.target;
      while (refElement.parentElement != null) {
        if (refElement.id === "ref-marker-popup") {
          return;
        }
        refElement = refElement.parentElement;
      }
      this.closeAction();
    },
    handleDrag(e) {
      e.stopPropagation();
      e.preventDefault();
    },
    handleCancel(e) {
      e.stopPropagation();
      delete this.linesStore.getLineById(this.editStore.isEditing.id)
        .customSpeedLimits[this.pointOne.id];
    },
    handleMaxSpeedInput(value) {
      const customSpeedLimits = {
        ...this.linesStore.getLineById(this.editStore.isEditing.id)
          .customSpeedLimits,
      };
      customSpeedLimits[this.pointOne.id] = value;
      this.linesStore.updateLineValues(this.editStore.isEditing.id, {
        customSpeedLimits,
      });
    },
  },
  beforeUnmount() {
    window.removeEventListener("click", this.handleClickOutside);
  },
};
</script>

<style lang="scss">
.ref-marker-popup {
  width: 300px;
  padding: $space-ssm $space-md $space-lg $space-md;
  cursor: default;

  &__title {
    font-size: $font-md;
  }
  &__sliders {
    display: flex;
    align-items: center;
    justify-content: space-between;

    &__element {
      flex-grow: 1;
      margin-right: $space-sm;
    }
  }
}
</style>
