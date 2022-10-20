<template>
  <GondolaIcon v-if="type === 'gondola'" />
  <BusIcon v-if="type === 'bus'" />
  <TramIcon v-if="type === 'tram'" />
  <SubwayIcon v-if="type === 'subway'" />
  <TrainIcon v-if="type === 'train'" />
  <TuneVariantIcon v-if="type === 'custom'" />
</template>

<script>
import GondolaIcon from "vue-material-design-icons/Gondola.vue";
import BusIcon from "vue-material-design-icons/Bus.vue";
import TrainIcon from "vue-material-design-icons/Train.vue";
import TramIcon from "vue-material-design-icons/Tram.vue";
import SubwayIcon from "vue-material-design-icons/Subway.vue";
import TuneVariantIcon from "vue-material-design-icons/TuneVariant.vue";

export default {
  props: {
    type: String,
  },
  components: {
    GondolaIcon,
    BusIcon,
    TrainIcon,
    TramIcon,
    SubwayIcon,
    TuneVariantIcon,
  },
};
</script>

<style></style>
