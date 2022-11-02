<template>
  <div class="line-element">
    <ColorPicker
      v-if="selectColor && selectType == false"
      :initColor="line.color"
      :handleColorChange="updateColor"
      :closeAction="toggleColorPick"
    />
    <TypePicker
      v-if="selectType && selectColor == false"
      :line="line"
      :closeAction="toggleTypePicker"
    />
    <div class="line-element__data">
      <TooltipButton :handler="toggleColorPick" toolTip="Change line color">
        <IconLine :color="line.color" />
      </TooltipButton>
      <input
        v-if="editStore.isEditing == line"
        type="text"
        class="line-element__name-input grow"
        :value="line.name"
        @change="editName"
      />
      <span v-else class="grow">{{ line.name }}</span>
      <TooltipButton
        v-if="editStore.isEditing != line"
        :handler="toggleEditing"
        toolTip="Start editing line"
        class="line-element__edit"
      >
        <span>
          <PencilOutlineIcon />
        </span>
      </TooltipButton>
      <TooltipButton
        v-if="
          (editStore.isEditing == line &&
            linesStore.lines[line.id].pointIds.length > 1) ||
          findStationLoading
        "
        :handler="findStation"
        toolTip="Automatically find ideal station"
      >
        <LoadingIcon v-if="findStationLoading" class="loader" />
        <BusStopIcon v-else />
      </TooltipButton>
      <TooltipButton
        v-if="editStore.isEditing != line"
        toolTip="Toggle line options"
        :handler="toggleTypePicker"
      >
        <TypeIcon :type="line.type" />
      </TooltipButton>
      <TooltipButton
        v-if="editStore.isEditing == line"
        toolTip="Remove line (permanent)"
        :handler="removeLine"
      >
        <span class="line-element__remove"><TrashCanOutlineIcon /></span>
      </TooltipButton>
    </div>
    <div
      class="line-element__warning"
      v-if="editStore.isEditing === line && editStore.isExtending != null"
    >
      <span class="line-element__warning__title"
        >{{ line.name }} is being extended!</span
      >
      <span>To end the line stop editing (Enter) <ArrowDownRightIcon /></span>
    </div>
  </div>
</template>

<script>
import PencilOutlineIcon from "vue-material-design-icons/PencilOutline.vue";
import TrashCanOutlineIcon from "vue-material-design-icons/TrashCanOutline.vue";
import ArrowDownRightIcon from "vue-material-design-icons/ArrowDownRight.vue";
import BusStopIcon from "vue-material-design-icons/BusStop.vue";
import LoadingIcon from "vue-material-design-icons/Loading.vue";
import TooltipButton from "./TooltipButton.vue";
import IconLine from "./IconLine.vue";
import { useLinesStore } from "../stores/lines";
import { useEditStore } from "../stores/editing";
import ColorPicker from "./ColorPicker.vue";
import { usePaxStore } from "../stores/pax";
import TypePicker from "./TypePicker.vue";
import TypeIcon from "./TypeIcon.vue";

export default {
  props: {
    line: Object,
  },
  components: {
    IconLine,
    BusStopIcon,
    PencilOutlineIcon,
    TrashCanOutlineIcon,
    ColorPicker,
    ArrowDownRightIcon,
    LoadingIcon,
    TooltipButton,
    TypePicker,
    TypeIcon,
},
  data() {
    return {
      linesStore: useLinesStore(),
      editStore: useEditStore(),
      paxStore: usePaxStore(),
      selectColor: false,
      selectType: false,
      findStationLoading: false,
    };
  },
  methods: {
    editName(e) {
      this.linesStore.getLineById(this.line.id).name = e.srcElement.value;
    },
    updateColor(color) {
      // Matomo Tracking
      if (window && window.Piwik) {
        window.Piwik.getTracker().trackEvent("editing", "updateColor", color);
      }
      this.linesStore.updateLineValues(this.line.id, { color });
    },
    toggleEditing() {
      // Matomo Tracking
      if (window && window.Piwik) {
        window.Piwik.getTracker().trackEvent("editing", "toggleEditing");
      }
      this.editStore.isEditing = this.line;
      if (this.line.pointIds.length === 0) {
        this.editStore.isExtending = -1;
        return;
      }
      this.editStore.isExtending = null;
    },
    toggleColorPick(e) {
      // Matomo Tracking
      if (window && window.Piwik) {
        window.Piwik.getTracker().trackEvent("editing", "toggleColorPick");
      }
      if (e) {
        e.stopPropagation();
      }
      this.selectType = false;
      this.selectColor = !this.selectColor;
    },
    toggleTypePicker(e) {
      // Matomo Tracking
      if (window && window.Piwik) {
        window.Piwik.getTracker().trackEvent("editing", "toggleTypePick");
      }
      if (e) {
        e.stopPropagation();
      }
      this.selectColor = false;
      this.selectType = !this.selectType;
    },
    removeLine() {
      // Matomo Tracking
      if (window && window.Piwik) {
        window.Piwik.getTracker().trackEvent("editing", "removeLine");
      }
      this.editStore.isEditing = null;
      this.editStore.isExtending = null;
      this.linesStore.removeLine(this.line);
    },
    findStation() {
      // Matomo Tracking
      if (window && window.Piwik) {
        window.Piwik.getTracker().trackEvent("editing", "findStation");
      }
      if (!this.findStationLoading) {
        this.findStationLoading = true;
        const route = [];
        const stations = [];
        const points = Object.values(this.linesStore.points);
        if (points.length <= 0) {
          return;
        }
        points.forEach((point) => {
          if (point.type === "station") {
            stations.push({
              lat: point.lat,
              lng: point.lng,
              id: point.id,
            });
          }
        });
        this.linesStore
          .getLineById(this.line.id)
          .pointIds.forEach((pointRef) => {
            const point = this.linesStore.getPointById(pointRef);
            route.push({
              lat: point.lat,
              lng: point.lng,
              id: point.id,
            });
          });

        fetch(import.meta.env.VITE_API_ENDPOINT + "/find-station", {
          method: "POST",
          body: JSON.stringify({
            route,
            stations,
            method: this.paxStore.calculationMethod,
          }),
          headers: {
            "Content-type": "application/json",
          },
        })
          .then((data) => data.json())
          .then((stationProposal) => {
            const newPoint = this.linesStore.addPoint(
              stationProposal.optimalStation.lat,
              stationProposal.optimalStation.lng,
              this.line,
              stationProposal.optimalStation.index
            );
            newPoint.type = "station";
          })
          .finally(() => {
            this.findStationLoading = false;
          });
      }
    },
  },
};
</script>

<style lang="scss">
.line-element {
  display: flex;
  flex-direction: column;
  margin-bottom: $space-ssm;

  &__data {
    display: flex;
    align-items: center;
    padding: $space-ssm $space-ssm;

    &:hover {
      .line-element__edit {
        display: block;
      }
    }
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
}
</style>
