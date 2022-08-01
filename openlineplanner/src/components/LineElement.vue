<template>
  <div class="line-element">
    <div class="line-element__data">
      <button @click="openColorPick"><IconLine :color="line.color" /></button>
      <ColorPicker
        v-if="selectColor == true"
        :initColor="line.color"
        :handleColorChange="updateColor"
        :closeAction="() => (selectColor = false)"
      />
      <input
        v-if="editStore.isEditing == line"
        type="text"
        class="line-element__name-input grow"
        :value="line.name"
        @change="editName"
      />
      <span v-else class="grow">{{ line.name }}</span>
      <button
        v-if="editStore.isEditing != line"
        class="line-element__edit"
        @click="toggleEditing"
      >
        <PencilOutlineIcon />
      </button>
      <button
        v-if="(editStore.isEditing == line && linesStore.lines[line.id].pointIds.length > 1) || findStationLoading"
        class="line-element__station"
        @click="findStation"
      >
        <LoadingIcon v-if="findStationLoading" class="loader" />
        <BusStopIcon v-else />
      </button>
      <button
        v-if="editStore.isEditing == line"
        class="line-element__remove"
        @click="removeLine"
      >
        <TrashCanOutlineIcon />
      </button>
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
    BusStopIcon,
    PencilOutlineIcon,
    TrashCanOutlineIcon,
    ColorPicker,
    ArrowDownRightIcon,
    LoadingIcon,
  },
  data() {
    return {
      linesStore: useLinesStore(),
      editStore: useEditStore(),
      selectColor: false,
      findStationLoading: false,
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
    findStation() {
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
          body: JSON.stringify({ route, stations }),
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
