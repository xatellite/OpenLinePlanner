<template>
  <div
    :class="`${
      isExtended ? 'view-settings view-settings--extended' : 'view-settings'
    }`"
    id="view-settings"
    @click="handleExtend"
  >
    <div class="view-settings__list" v-if="isExtended">
      <!-- <span>View Settings</span> -->
      <OverlaySelect />
      <CoverageSelect />
      <ToggleButton
        :active="overlayStore.showNameTags"
        :callback="overlayStore.toggleNameTags"
      >
        <template v-slot:icon>
          <BusStopIcon />
        </template>
        <template v-slot:text>
          <span>Station labels</span>
        </template>
      </ToggleButton>
      <ToggleButton
        :active="overlayStore.showDistanceTags"
        :callback="overlayStore.toggleDistanceTags"
      >
        <template v-slot:icon>
          <RulerIcon />
        </template>
        <template v-slot:text>
          <span>Distances</span>
        </template>
      </ToggleButton>
      <ToggleButton
        :active="paxStore.calculationMethod == 'relative'"
        :callback="paxStore.toggleCalculationMethod"
      >
        <template v-slot:icon>
          <BullseyeIcon v-if="paxStore.calculationMethod == 'relative'" />
          <CheckboxBlankCircleIcon v-else />
        </template>
        <template v-slot:text>
          <span v-if="paxStore.calculationMethod == 'relative'">Relative calc.</span>
          <span v-else>Absolute calc.</span>
        </template>
      </ToggleButton>
    </div>
    <CogOutlineIcon v-else class="view-settings__icon" />
  </div>
</template>

<script>
import CogOutlineIcon from "vue-material-design-icons/CogOutline.vue";
import BullseyeIcon from "vue-material-design-icons/Bullseye.vue";
import CheckboxBlankCircleIcon from "vue-material-design-icons/CheckboxBlankCircle.vue";
import BusStopIcon from "vue-material-design-icons/BusStop.vue";
import RulerIcon from "vue-material-design-icons/Ruler.vue";
import OverlaySelect from "./OverlaySelect.vue";
import ToggleButton from "./ToggleButton.vue";
import { useOverlayStore } from "../stores/overlay";
import { usePaxStore } from "../stores/pax";
import CoverageSelect from "./CoverageSelect.vue";

export default {
  components: {
    CogOutlineIcon,
    OverlaySelect,
    ToggleButton,
    BusStopIcon,
    RulerIcon,
    BullseyeIcon,
    CheckboxBlankCircleIcon,
    CoverageSelect,
},
  data() {
    return {
      isExtended: false,
      overlayStore: useOverlayStore(),
      paxStore: usePaxStore(),
    };
  },
  mounted() {
    window.addEventListener("click", this.handleClickOutside);
  },
  beforeUnmount() {
    window.removeEventListener("click", this.handleClickOutside);
  },
  methods: {
    handleClickOutside(e) {
      if (e.target != this.$el) {
        this.isExtended = false;
      }
    },
    handleExtend() {
      if (!this.isExtended) {
        this.isExtended = true;
      }
    },
  },
};
</script>

<style lang="scss" scoped>

.view-settings {
  position: absolute;
  z-index: 10;
  bottom: $space-slg;
  right: $space-md;
  padding: $space-sm $space-sm;
  max-width: $space-md;
  max-height: $space-md;
  background-color: $c-box;
  border-radius: $br-md;
  box-shadow: $bs-md;
  cursor: pointer;

  transition: max-height 0.4s linear;

  &--extended {
    max-width: 300px;
    max-height: 600px;
  }

  &__list {
    display: flex;
    flex-direction: column;

    & > * {
      margin: $space-ssm 0;
    }
  }

  &__icon {
    pointer-events: none;
    cursor: pointer;
  }
}
</style>
