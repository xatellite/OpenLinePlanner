<template>
  <div class="line-element">
    <div class="line-element__data">
      <TooltipButton
        toolTip="Toggle line options"
        :handler="toggleTypePicker"
      >
        <TypeIcon :type="line.type" />
      </TooltipButton>
      <IconLine @click="toggleColorPick" :color="line.color" />
      <input
        v-if="editStore.isEditing == line"
        type="text"
        class="line-element__name-input grow"
        :value="line.name"
        @change="editName"
      />
      <span v-else class="grow">{{ line.name }}</span>
      <TooltipButton
        v-if="editStore.isEditing == line"
        toolTip="Remove line (permanent)"
        :handler="removeLine"
      >
        <span class="remove"><TrashCanOutlineIcon /></span>
      </TooltipButton>
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
    </div>
    <Teleport to="#map" v-if="editStore.isEditing == line && (selectColor || selectType)">
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
    </Teleport>

    <div
      class="line-element__warning"
      v-if="editStore.isEditing === line"
      @click="disableEditing"
    >
      <span class="line-element__warning__title"
        >{{ line.name }} is being {{ editStore.isExtending ? "extended": "edited"}}</span
      >
      <span>Click here to stop {{ editStore.isExtending ? "extending (⏎)": "editing"}}</span>
    </div>
  </div>
</template>

<script>
import PencilOutlineIcon from "vue-material-design-icons/PencilOutline.vue";
import TrashCanOutlineIcon from "vue-material-design-icons/TrashCanOutline.vue";
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
    LoadingIcon,
    PencilOutlineIcon,
    BusStopIcon,
    ColorPicker,
    TooltipButton,
    TypePicker,
    TypeIcon,
    TrashCanOutlineIcon,
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
  mounted() {
    window.addEventListener("keydown", (e) => {
      if (this.editStore.isExtending) {
        if (e.key === "Enter") {
          this.disableEditing();
        }
      }
    });
  },
  methods: {
    disableEditing() {
      this.editStore.isEditing = null;
      this.editStore.isExtending = null;
    },
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
      if (this.editStore.isEditing != this.line) {
        this.selectColor = false;
      }
      this.toggleEditing();
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
      if (this.editStore.isEditing != this.line) {
        this.selectType = false;
      }
      this.toggleEditing();
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
        // ToDo: Add max coverage
        points.forEach((point) => {
          if (point.type === "station") {
            stations.push({
              location: {
                y: point.lat,
                x: point.lng,
              },
              id: point.id,
              // Add max coverage
              coverage: Math.max(
                ...point.lines.map((lineId) =>
                  this.linesStore.getLineById(lineId).getCoverage()
                )
              ),
            });
          }
        });
        this.linesStore
          .getLineById(this.line.id)
          .pointIds.forEach((pointRef) => {
            const point = this.linesStore.getPointById(pointRef);
            route.push({
              y: point.lat,
              x: point.lng,
            });
          });
        fetch(import.meta.env.VITE_API_ENDPOINT + "/find-station", {
          method: "POST",
          body: JSON.stringify({
            route,
            stations,
            coverage: this.editStore.isEditing.getCoverage(),
            method: this.paxStore.calculationMethod,
          }),
          headers: {
            "Content-type": "application/json",
          },
        })
          .then((data) => data.json())
          .then((stationProposal) => {
            const newPoint = this.linesStore.addPoint(
              stationProposal.location.y,
              stationProposal.location.x,
              this.line,
              stationProposal.index
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

<style lang="scss" scoped>
.line-element {
  display: flex;
  flex-direction: column;

  &__data {
    display: flex;
    align-items: center;
    padding: $space-ssm $space-ssm;
    gap: $space-ssm;

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
    color: var(--c-text-inverted);
    background-color: var(--c-accent-two);
    cursor: pointer;
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
    font-family: "Poppins";
    font-size: $font-md;
    color: var(--c-primary-light);
    width: 0;
  }

  .grow {
    flex-grow: 1;
    padding: $space-ssm;
  }

  &__edit {
    display: none;
  }
}
</style>
