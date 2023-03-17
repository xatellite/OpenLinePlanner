<template>
  <div class="box data-overlay" v-if="dataStore.addPoint">
    <span></span>
    <ListContainer title="Import Options">
      <div class="data-overlay__import-entry" v-for="method, index in dataStore.methods" :key="`data-method-${index}`">
        <TooltipButton tooltip="Select layer import" :handler="() => dataStore.selectMethod(method)">
          <PlusIcon />
        </TooltipButton>
        <div class="data-overlay__import-entry__text">
          <span class="data-overlay__import-entry__text__title">{{ method.title }}</span>
          <span>{{ method.description }}</span>
        </div>
      </div>
    </ListContainer>
  </div>
</template>

<script>
import PlusIcon from "vue-material-design-icons/Plus.vue";
import { useDataStore } from '../stores/data';
import ListContainer from './ListContainer.vue';
import TooltipButton from './TooltipButton.vue';

export default {
    data() {
        return {
            dataStore: useDataStore(),
        };
    },
    components: { PlusIcon, ListContainer, TooltipButton }
}
</script>

<style lang="scss" scoped>
.data-overlay {
  position: absolute;
  bottom: $space-sm;
  left: $space-sm;
  z-index: 3;
  padding: $space-sm;

  &__import-entry {
    display: flex;
    gap: $space-sm;
    padding: $space-ssm $space-sm;
    margin: $space-ssm;
    border-radius: $br-md;
    background-color: $c-box;
    border: 1px solid $c-button-border;

    &__text {
      display: flex;
      flex-direction: column;

      &__title {
        font-size: $font-md;
      }
    }
  }
}
</style>