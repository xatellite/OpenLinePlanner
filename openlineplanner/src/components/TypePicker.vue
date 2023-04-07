<template>
  <div id="type-picker" class="box type-picker">
    <div class="type-picker__base">
      <span class="type-picker__title"
        >Select type of line: <b>{{ line.name }}</b></span
      >
      <div class="type-picker__options">
        <TooltipButton
          :active="line.type === 'gondola'"
          @click="setType('gondola')"
          toolTip="Gondola"
        >
          <GondolaIcon />
        </TooltipButton>
        <TooltipButton
          :active="line.type === 'bus'"
          @click="setType('bus')"
          toolTip="Bus"
        >
          <BusIcon />
        </TooltipButton>
        <TooltipButton
          :active="line.type === 'tram'"
          @click="setType('tram')"
          toolTip="Tram"
        >
          <TramIcon />
        </TooltipButton>
        <TooltipButton
          :active="line.type === 'subway'"
          @click="setType('subway')"
          toolTip="Subway"
        >
          <SubwayIcon />
        </TooltipButton>
        <TooltipButton
          :active="line.type === 'train'"
          @click="setType('train')"
          toolTip="Train"
        >
          <TrainIcon />
        </TooltipButton>
        <TooltipButton
          :active="line.type === 'custom'"
          @click="setType('custom')"
          toolTip="Custom"
        >
          <TuneVariantIcon />
        </TooltipButton>
      </div>
    </div>
    <div class="type-picker__sliders">
      <div class="type-picker__sliders__title">
        <span>Coverage Area: </span><b>{{ this.coverageArea }}m</b>
      </div>
      <VueSlider
        class="type-picker__sliders__element"
        :modelValue="this.coverageArea"
        :disabled="line.type !== 'custom'"
        :max="2000"
        :min="50"
        tooltip="none"
        :marks="[50, 250, 500, 750, 1000, 1250, 1500, 1750, 2000]"
        @update:modelValue="updateValue({ customCoverage: $event })"
      />
      <div class="type-picker__sliders__title">
        <span>Avg. acceleration: </span><b>{{ this.acceleration }}m/s²</b>
      </div>
      <VueSlider
        class="type-picker__sliders__element"
        :modelValue="this.acceleration"
        :disabled="line.type !== 'custom'"
        :interval="0.05"
        :max="5"
        tooltip="none"
        :marks="[0, 0.5, 1, 1.5, 2, 2.5, 3, 3.5, 4, 4.5, 5]"
        @update:modelValue="updateValue({ customAcceleration: $event })"
      />
      <div class="type-picker__sliders__title">
        <span>Avg. stop time: </span><b>{{ this.stopTime }}s</b>
      </div>
      <VueSlider
        class="type-picker__sliders__element"
        :modelValue="this.stopTime"
        :disabled="line.type !== 'custom'"
        :max="600"
        tooltip="none"
        :marks="[0, 60, 120, 180, 240, 300, 360, 420, 480, 540, 600]"
        @update:modelValue="updateValue({ customStopTime: $event })"
      />
      <div class="type-picker__sliders__title">
        <span>General max Speed: </span><b>{{ this.maxSpeed }}km/h</b>
      </div>
      <VueSlider
        class="type-picker__sliders__element"
        :modelValue="this.maxSpeed"
        :disabled="line.type !== 'custom'"
        :max="200"
        tooltip="none"
        :marks="[0, 30, 50, 100, 150, 200]"
        @update:modelValue="updateValue({ customMaxSpeed: $event })"
      />
      <div class="type-picker__sliders__title">
        <span>Line width: </span><b>{{ this.lineWidth }}px</b>
      </div>
      <VueSlider
        class="type-picker__sliders__element"
        :modelValue="this.lineWidth"
        :disabled="line.type !== 'custom'"
        :max="8"
        :min="4"
        :marks="true"
        tooltip="none"
        @update:modelValue="updateValue({ lineThickness: $event })"
      />
    </div>
  </div>
</template>

<script>
import GondolaIcon from "vue-material-design-icons/Gondola.vue";
import BusIcon from "vue-material-design-icons/Bus.vue";
import TrainIcon from "vue-material-design-icons/Train.vue";
import TramIcon from "vue-material-design-icons/Tram.vue";
import SubwayIcon from "vue-material-design-icons/Subway.vue";
import TuneVariantIcon from "vue-material-design-icons/TuneVariant.vue";
import TooltipButton from "./TooltipButton.vue";
import { useLinesStore } from "../stores/lines";
import { useEditStore } from "../stores/editing";
import TransportLine from "../helpers/classes/TransportLine";
import VueSlider from "vue-slider-component";
import LineTypeParameters from "../assets/defaults/lineTypes.json";
import { usePaxStore } from "../stores/pax";

export default {
  components: {
    GondolaIcon,
    BusIcon,
    TrainIcon,
    TramIcon,
    SubwayIcon,
    TuneVariantIcon,
    TooltipButton,
    VueSlider,
  },
  props: {
    line: TransportLine,
    closeAction: Function,
  },
  data() {
    return {
      linesStore: useLinesStore(),
      editStore: useEditStore(),
      paxStore: usePaxStore(),
      acceleration: this.line.customAcceleration,
      stopTime: this.line.customStopTime,
      maxSpeed: this.line.customMaxSpeed,
      lineWidth: this.line.lineThickness,
      coverageArea: this.line.customCoverage,
    };
  },
  mounted() {
    window.addEventListener("click", this.handleClickOutside);
  },
  methods: {
    setType(typeString) {
      this.linesStore.updateLineValues(this.line.id, { type: typeString });
      if (typeString !== "custom") {
        this.acceleration = LineTypeParameters[typeString]["acceleration"];
        this.stopTime = LineTypeParameters[typeString]["stopTime"];
        this.maxSpeed = LineTypeParameters[typeString]["maxSpeed"];
        this.lineWidth = LineTypeParameters[typeString]["lineWidth"];
        this.coverageArea = LineTypeParameters[typeString]["coverageArea"];
      } else {
        this.acceleration = this.line["customAcceleration"];
        this.stopTime = this.line["customStopTime"];
        this.maxSpeed = this.line["customMaxSpeed"];
        this.lineWidth = this.line["lineThickness"];
        this.coverageArea = this.line["customCoverage"];
      }
      // Tell paxStore to update station info and overlays
      this.paxStore.setCurrent();
    },
    updateValue(updateObject) {
      this.linesStore.updateLineValues(this.line.id, updateObject);
      this.acceleration = this.line["customAcceleration"];
      this.stopTime = this.line["customStopTime"];
      this.maxSpeed = this.line["customMaxSpeed"];
      this.lineWidth = this.line["lineThickness"];
      this.coverageArea = this.line["customCoverage"];
      if (updateObject["customCoverage"]) {
        this.paxStore.setCurrent();
      }
    },
    handleClickOutside(e) {
      let refElement = e.target;
      while (refElement.parentElement != null) {
        if (refElement.id === "type-picker") {
          return;
        }
        refElement = refElement.parentElement;
      }
      this.closeAction();
    },
  },
  beforeUnmount() {
    window.removeEventListener("click", this.handleClickOutside);
  },
};
</script>

<style lang="scss">
$themeColor: var(--c-primary-light);
@import "vue-slider-component/lib/theme/default.scss";

.type-picker {
  position: absolute;
  font-size: $font-md;
  top: $space-sm;
  left: 0;
  z-index: 100;
  margin: 0 $space-sm;

  @media (max-width: 700px), (max-height: 600px) {
    position: relative;
    left: auto;
  }

  &__base {
    display: flex;
    flex-direction: column;
    padding: $space-sm;
    border-bottom: 1px solid rgba(var(--c-primary-light), 0.2);
  }

  &__options {
    display: flex;
    gap: $space-sm;
    justify-content: space-between;
  }

  &__title {
    margin: $space-ssm 0;
  }
  &__sliders {
    padding: $space-sm $space-sm;

    &__element {
      margin: $space-ssm $space-sm $space-lg;
    }

    &__title {
      display: flex;
      justify-content: space-between;
    }
  }
}
</style>
