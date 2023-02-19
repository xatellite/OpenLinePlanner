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
        <span class="line-element__remove"><TrashCanOutlineIcon /></span>
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
    </div>
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
    PencilOutlineIcon,
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
  },
};
</script>

<style lang="scss">
.line-element {
  display: flex;
  flex-direction: column;

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
    color: $c-text-inverted;
    background-color: $c-accent-two;
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
    color: $c-text-primary;
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
