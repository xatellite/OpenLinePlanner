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
        toolTip="Toggle Station names"
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
        toolTip="Toggle line distances"
      >
        <template v-slot:icon>
          <RulerIcon />
        </template>
        <template v-slot:text>
          <span>Distances</span>
        </template>
      </ToggleButton>
      <ToggleButton
        :active="overlayStore.showSpeedTags"
        :callback="overlayStore.toggleSpeedTags"
        toolTip="Toggle line max speed"
      >
        <template v-slot:icon>
          <SpeedometerIcon />
        </template>
        <template v-slot:text>
          <span>Max speed</span>
        </template>
      </ToggleButton>
      <ToggleButton
        :active="paxStore.calculationMethod == 'relative'"
        :callback="paxStore.toggleCalculationMethod"
        toolTip="Switch calculation method"
      >
        <template v-slot:icon>
          <BullseyeIcon v-if="paxStore.calculationMethod == 'relative'" />
          <CheckboxBlankCircleIcon v-else />
        </template>
        <template v-slot:text>
          <span v-if="paxStore.calculationMethod == 'relative'"
            >Relative calc.</span
          >
          <span v-else>Absolute calc.</span>
        </template>
      </ToggleButton>
      <ToggleButton
        :active="paxStore.routingMethod == 'relative'"
        :callback="paxStore.toggleRoutingMethod"
        toolTip="Switch routing method"
      >
        <template v-slot:icon>
          <MapIcon v-if="paxStore.routingMethod == 'osm'" />
          <MapMarkerDistanceIcon v-else />
        </template>
        <template v-slot:text>
          <span v-if="paxStore.routingMethod == 'naive'">Radius routing</span>
          <span v-else>OSM routing</span>
        </template>
      </ToggleButton>
    </div>
    <LayersOutlineIcon v-else class="view-settings__icon" />
  </div>
</template>

<script>
import LayersOutlineIcon from "vue-material-design-icons/LayersOutline.vue";
import BullseyeIcon from "vue-material-design-icons/Bullseye.vue";
import MapMarkerDistanceIcon from "vue-material-design-icons/MapMarkerDistance.vue";
import MapIcon from "vue-material-design-icons/Map.vue";
import CheckboxBlankCircleIcon from "vue-material-design-icons/CheckboxBlankCircle.vue";
import BusStopIcon from "vue-material-design-icons/BusStop.vue";
import RulerIcon from "vue-material-design-icons/Ruler.vue";
import SpeedometerIcon from "vue-material-design-icons/Speedometer.vue";
import OverlaySelect from "./OverlaySelect.vue";
import ToggleButton from "./ToggleButton.vue";
import { useOverlayStore } from "../stores/overlay";
import { usePaxStore } from "../stores/pax";
import CoverageSelect from "./CoverageSelect.vue";

export default {
  components: {
    LayersOutlineIcon,
    OverlaySelect,
    ToggleButton,
    BusStopIcon,
    RulerIcon,
    BullseyeIcon,
    MapIcon,
    MapMarkerDistanceIcon,
    CheckboxBlankCircleIcon,
    CoverageSelect,
    SpeedometerIcon,
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
  bottom: $space-md;
  right: $space-sm;
  padding: $space-sm $space-sm;
  max-width: $space-md;
  max-height: $space-md;
  background-color: var(--c-box);
  border-radius: $br-md;
  box-shadow: var(--bs-md);
  font-size: $font-md;
  cursor: pointer;

  transition: max-height 0.4s linear;

  @media (max-width: 700px), (max-height: 600px) {
    top: $space-sm;
    bottom: auto;
    left: $space-sm;
  }

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
