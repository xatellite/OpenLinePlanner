<template>
  <div
    id="ref-marker-popup"
    class="box ref-marker-popup"
    @mousedown="handleMouseDown"
  >
    <div class="ref-marker-popup__input">
      <span class="ref-marker-popup__input__title">Section max Speed:</span>
      <input
        type="number"
        :value="
          linesStore
            .getLineById(editStore.isEditing.id)
            .getSectionSpeed(pointOne.id)
        "
        max="200"
        min="0"
        @change="handleMaxSpeedInput"
        @mousedown="handleMouseDown"
      />
      <b>km/h</b>
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

export default {
  props: {
    pointOne: Object,
    closeAction: Function,
  },
  components: { TooltipButton, CancelIcon },
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
    handleMouseDown(e) {
      e.stopPropagation();
    },
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
    handleCancel(e) {
      e.stopPropagation();
      delete this.linesStore.getLineById(this.editStore.isEditing.id)
        .customSpeedLimits[this.pointOne.id];
    },
    handleMaxSpeedInput(e) {
      console.log(e);
      const value = e.target.value;
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

<style lang="scss" scoped>
.ref-marker-popup {
  width: fit-content;
  padding: $space-sm $space-sm $space-sm $space-sm;
  cursor: default;

  &__input {
    position: relative;
    background-color: var(--c-box);
    padding: $space-ssm $space-sm;
    border: 1px solid var(--c-button-border);
    border-radius: $br-md;
    margin-top: $space-ssm;

    display: flex;
    align-items: center;
    justify-content: flex-start;

    &:focus-within {
      outline: 1px solid var(--c-accent-two);

      .text-input__title {
        color: var(--c-accent-two);
      }
    }

    &__title {
      position: absolute;
      top: -0.8em;
      color: var(--c-primary-light);
      background-color: var(--c-box);
      left: $space-sm;
      padding: 0 $space-ssm;
    }

    & > * {
      margin-right: $space-sm;

      &:focus {
        outline: none;
      }
    }

    &__element {
      flex-grow: 1;
      margin-right: $space-sm;
    }
  }
}
</style>
