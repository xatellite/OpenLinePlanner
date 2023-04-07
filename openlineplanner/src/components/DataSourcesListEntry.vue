<template>
  <div class="layer-entry">
    <HomeCityIcon v-if="layer.type == 'residential'" />
    <CartOutlineIcon v-else-if="layer.type == 'shopping'" />
    <BriefcaseOutlineIcon v-else-if="layer.type == 'business'" />
    <SchoolOutlineIcon v-else-if="layer.type == 'school'" />
    <TheaterIcon v-else-if="layer.type == 'leisure'" />
    <BinocularsIcon v-else-if="layer.type == 'tourism'" />
    <MapMarkerStarOutline v-else />
    <div class="layer-entry__text">
      <span class="layer-entry__text__title">{{ layer.properties.name }}</span>
      <span>{{ layer.properties.description }}</span>
    </div>
    <TooltipButton
      toolTip="Remove layer (permanent)"
      :handler="dataStore.removeLayer(layer)"
    >
      <span class="remove"><TrashCanOutlineIcon /></span>
    </TooltipButton>
  </div>
</template>

<script>
import HomeCityIcon from "vue-material-design-icons/HomeCity.vue";
import CartOutlineIcon from "vue-material-design-icons/CartOutline.vue";
import BriefcaseOutlineIcon from "vue-material-design-icons/BriefcaseOutline.vue";
import SchoolOutlineIcon from "vue-material-design-icons/SchoolOutline.vue";
import TheaterIcon from "vue-material-design-icons/Theater.vue";
import BinocularsIcon from "vue-material-design-icons/Binoculars.vue";
import MapMarkerStarOutline from "vue-material-design-icons/MapMarkerStarOutline.vue";
import TrashCanOutlineIcon from "vue-material-design-icons/TrashCanOutline.vue";
import { useDataStore } from '../stores/data';

export default {
  props: {
    layer: Object,
  },
  data() {
    return {
      dataStore: useDataStore(),
    };
  },
  components: {
    HomeCityIcon,
    CartOutlineIcon,
    BriefcaseOutlineIcon,
    SchoolOutlineIcon,
    TheaterIcon,
    BinocularsIcon,
    MapMarkerStarOutline,
    TrashCanOutlineIcon,
  },
};
</script>

<style lang="scss" scoped>
.line-element {
  display: flex;
  gap: $space-sm;

  &__text {
    display: flex;
    flex-direction: column;

    &__title {
      font-weight: bold;
    }
  }
}
</style>
