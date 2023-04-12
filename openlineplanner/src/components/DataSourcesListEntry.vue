<template>
  <div
    :class="layerActive ? 'layer-entry layer-entry--active' : 'layer-entry'"
    @click="toggleLayer"
  >
    <HomeCityIcon v-if="layer.layer_type == 'Residential'" />
    <CartOutlineIcon v-else-if="layer.layer_type == 'Shopping'" />
    <BriefcaseOutlineIcon v-else-if="layer.layer_type == 'Business'" />
    <SchoolOutlineIcon v-else-if="layer.layer_type == 'School'" />
    <TheaterIcon v-else-if="layer.layer_type == 'Leisure'" />
    <BinocularsIcon v-else-if="layer.layer_type == 'Tourism'" />
    <MapMarkerStarOutline v-else />
    <div class="layer-entry__text">
      <span class="layer-entry__text__title">{{ layer.name }}</span>
      <span>{{ layer.layer_type }}</span>
    </div>
    <LoadingIcon v-if="layerLoading" class="loader" />
    <TooltipButton
      toolTip="Remove layer (permanent)"
      :handler="remove"
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
import LoadingIcon from "vue-material-design-icons/Loading.vue";
import { useDataStore } from "../stores/data";
import TooltipButton from "./TooltipButton.vue";
import { getLayer } from '../helpers/api';

export default {
  props: {
    layer: Object,
  },
  data() {
    return {
      dataStore: useDataStore(),
      layerActive: false,
      layerLoading: false,
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
    TooltipButton,
    LoadingIcon,
  },
  methods: {
    toggleLayer() {
      if (this.layerLoading) {
        return;
      }
      if (this.layerActive) {
        this.dataStore.removeMapBoundsById(this.layer.id);
        this.layerActive = false;
        return;
      }
      this.layerActive = true;
      this.layerLoading = true;
      getLayer(this.layer.id).then((layerGeoJson) => {
        layerGeoJson.properties = {};
        layerGeoJson.properties.id = this.layer.id;
        this.dataStore.addMapBound(layerGeoJson, "points", "layer", "#FEC80B");
        this.layerLoading = false;
      });
    },
    remove(event) {
      event.stopPropagation();
      this.dataStore.removeLayer(this.layer);
    },
  }
};
</script>

<style lang="scss" scoped>
.layer-entry {
  display: flex;
  gap: $space-sm;
  padding: $space-sm $space-md;
  align-items: center;
  cursor: pointer;

  &--active {
    background-color: var(--c-background-inverted);
    color: var(--c-text-inverted);
  }

  &__text {
    display: flex;
    flex-direction: column;
    font-size: $font-sm;
    margin-right: auto;

    &__title {
      font-weight: bold;
      font-size: $font-md;
    }
  }
}
</style>
