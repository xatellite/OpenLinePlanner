<template>
  <div @click="handlePopupOpen" class="line-reference-point">
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
        this.popupOpen = true;
      }
    },
  },
};
</script>

<style lang="scss">
.line-reference-point {
  position: relative;
  border: 12px solid transparent;
  border-radius: 100%;
  width: 10px;
  height: 10px;
  cursor: pointer;

  &__distance {
    position: absolute;
    font-weight: 700;
    left: -$space-md;
    top: $space-sm;
    background: $c-box;
    border-radius: $br-md;
    padding: 0 $space-ssm;
  }

  &__speed {
    position: absolute;
    font-weight: 700;
    left: -$space-md;
    top: -($space-sm + $space-sm);
    background: $c-box;
    border-radius: $br-md;
    padding: 0 $space-ssm;
  }

  &__popup {
    position: absolute;
    z-index: 100;
    bottom: 100%;
    left: -$space-sm;
  }

  &::before {
    display: block;
    content: "";
    height: 8px;
    width: 8px;
    border-radius: 100%;
    background-color: rgba($c-box, 0.5);
  }
}
</style>
