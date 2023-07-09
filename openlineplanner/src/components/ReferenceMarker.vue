<template>
  <div @click="handlePopupOpen" class="line-reference-point">
    <div class="line-reference-point__marker" :style="{backgroundColor: line.color}"></div>
    <span
      class="line-reference-point__distance"
      v-if="overlayStore.showDistanceTags"
      >{{ lngLat.totalDistance }}m</span
    >
    <span
      v-if="overlayStore.showSpeedTags && editStore.isEditing && !popupOpen"
      class="line-reference-point__speed"
      >{{
        linesStore
          .getLineById(editStore.isEditing.id)
          .getSectionSpeed(pointOne.id)
      }}km/h</span
    >
    <ReferenceMarkerPopup
      class="line-reference-point__popup"
      v-if="popupOpen && editStore.isEditing"
      :closeAction="() => popupOpen = false"
      :pointOne="pointOne"
    />
  </div>
</template>

<script>
import { useOverlayStore } from "../stores/overlay";
import { useEditStore } from "../stores/editing";
import { useLinesStore } from "../stores/lines";
import ReferenceMarkerPopup from "./ReferenceMarkerPopup.vue";

export default {
  components: { ReferenceMarkerPopup },
  props: {
    pointOne: Object,
    pointTwo: Object,
    lngLat: Object,
    line: Object,
  },
  data: function () {
    return {
      overlayStore: useOverlayStore(),
      editStore: useEditStore(),
      linesStore: useLinesStore(),
      popupOpen: false,
    };
  },

  methods: {
    handlePopupOpen(event) {
      if (!this.popupOpen) {
        event.stopPropagation();
        this.editStore.isExtending = null;
        this.popupOpen = true;
      }
    },
  },
};
</script>

<style lang="scss" scoped>
.line-reference-point {
  position: relative;
  border: 12px solid transparent;
  border-radius: 100%;
  box-sizing: content-box;
  padding: 20px;
  height: 10px;
  width: 10px;
  //height: 10px;

  &:hover {
    // background-color: aliceblue;
    .line-reference-point__marker {
      display: block;
    }
  }

  &__distance {
    position: absolute;
    font-weight: 700;
    left: -5px;
    bottom: 0;
    background: var(--c-box);
    border-radius: $br-md;
    padding: 0 $space-ssm;
  }

  &__speed {
    position: absolute;
    font-weight: 700;
    left: -5px;
    top: 0;
    background: var(--c-box);
    border-radius: $br-md;
    padding: 0 $space-ssm;
  }

  &__popup {
    position: absolute;
    z-index: 100;
    bottom: 100%;
    left: 0;
  }

  &__marker {
    display: none;
    position: absolute;
    height: 8px;
    width: 8px;
    border-radius: 100%;
    pointer-events: auto;
    border: 2px solid var(--c-box);
    cursor: pointer;
  }
}
</style>
