<template>
  <div class="box data-overlay" v-if="dataStore.addPoint">
    <!-- Select Method -->
    <div class="data-overlay__box" v-if="!dataStore.selectedMethod">
      <span class="data-overlay__title">Select Import</span>
      <ListContainer title="Import Options">
        <div
          class="data-overlay__import-entry"
          v-for="(method, index) in dataStore.methods"
          :key="`data-method-${index}`"
        >
          <TooltipButton
            toolTip="Select layer import"
            :handler="() => dataStore.selectMethod(method)"
          >
            <PlusIcon />
          </TooltipButton>
          <div class="data-overlay__import-entry__text">
            <span class="data-overlay__import-entry__text__title">{{
              method.title
            }}</span>
            <span>{{ method.description }}</span>
          </div>
        </div>
      </ListContainer>
    </div>
    <!-- Select Area Screen -->
    <div
      class="data-overlay__box"
      v-if="dataStore.selectedMethod && !dataStore.selectedArea"
    >
      <span class="data-overlay__title">Select Area</span>
      <ListContainer title="Available Areas">
        <div
          class="data-overlay__import-entry"
          v-for="(area, index) in Object.values(dataStore.areas)"
          :key="`data-method-${index}`"
        >
          <TooltipButton
            tooltip="Select layer import"
            :handler="() => selectArea(area)"
          >
            <SelectionEllipseIcon
              class="data-overlay__import-entry__selected"
              v-if="area == selectedArea"
            />
          </TooltipButton>
          <div class="data-overlay__import-entry__text">
            <span class="data-overlay__import-entry__text__title">{{
              area.properties.name
            }}</span>
          </div>
        </div>
      </ListContainer>
      <div class="data-overlay__actions">
        <button @click="dataStore.selectedMethod = null">Back</button>
        <button
          @click="dataStore.selectArea(selectArea)"
          :disabled="!selectedArea"
        >
          Next
        </button>
      </div>
    </div>
    <!-- Answer Questions -->
    <div class="data-overlay__box" v-if="dataStore.selectedArea">
      <span class="data-overlay__title">Data Needed</span>
      <div>
        <div
          v-for="(question, index) in dataStore.selectedMethod.questions"
          :key="`question-${index}`"
        >
          <div v-if="question.type == 'number'">
            <TextInput
              :title="question.text"
              type="number"
              :value="question.answer"
              :callback="(value) => dataStore.answerQuestion(index, value)"
            />
          </div>
          <div v-if="question.type == 'bool'">
            <span>question.text</span>
            <input type="text" />
          </div>
        </div>
      </div>
      <div class="data-overlay__actions">
        <button @click="dataStore.selectedArea = null">Back</button>
        <button :disabled="!dataStore.questionsResolved">Request</button>
      </div>
    </div>
  </div>
</template>

<script>
import PlusIcon from "vue-material-design-icons/Plus.vue";
import SelectionEllipseIcon from "vue-material-design-icons/SelectionEllipse.vue";
import { useDataStore } from "../stores/data";
import ListContainer from "./ListContainer.vue";
import TextInput from "./TextInput.vue";
import TooltipButton from "./TooltipButton.vue";

export default {
  data() {
    return {
      dataStore: useDataStore(),
      selectedArea: null,
      questionsResolved: false,
    };
  },
  components: {
    PlusIcon,
    ListContainer,
    TooltipButton,
    TextInput,
    SelectionEllipseIcon,
  },
  methods: {
    selectArea(area) {
      this.dataStore.highlightArea(area);
      this.selectedArea = area;
    },
  },
};
</script>

<style lang="scss" scoped>
.data-overlay {
  position: absolute;
  bottom: $space-sm;
  left: $space-sm;
  z-index: 3;
  padding: $space-sm;

  &__box {
    display: flex;
    flex-direction: column;
  }

  &__title {
    font-size: $font-md;
    font-weight: bold;
    padding: 0 0 $space-ssm;
  }

  &__actions {
    display: flex;
    padding: $space-ssm 0;
    justify-content: space-between;

    button {
      width: auto;
      padding: $space-ssm $space-sm;
    }
  }

  &__import-entry {
    display: flex;
    gap: $space-sm;
    padding: $space-ssm $space-sm;
    margin: $space-ssm;
    border-radius: $br-md;
    background-color: $c-box;
    border: 1px solid $c-button-border;

    &__selected {
      color: $c-accent-two;
    }

    &__text {
      display: flex;
      flex-direction: column;
      justify-content: center;

      &__title {
        font-size: $font-md;
      }
    }
  }
}
</style>
