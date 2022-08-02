<template>
  <div :class="overlayStore.overlay != 'none' ?'select-container select-container--active':'select-container'" @click="handleClick">
    <span>
      <LoadingIcon class="loader" v-if="overlayStore.overlay != 'none' && Object.keys(overlayStore.overlayData).length === 0" />
      <HomeAccountIcon v-else-if="overlayStore.overlay == 'residence'" />
      <SchoolOutlineIcon v-else-if="overlayStore.overlay == 'schools'" />
      <BriefcaseAccountIcon v-else-if="overlayStore.overlay == 'jobs'" />
      <BlurOffIcon v-else />
    </span>
    <select @change="handleSelect" :value="overlayStore.overlay">
      <option value="none">None</option>
      <option value="residence">Residential</option>
      <option value="schools">Schools</option>
      <option value="jobs">Jobs/Leisure</option>
    </select>
  </div>
</template>

<script>
import HomeAccountIcon from "vue-material-design-icons/HomeAccount.vue";
import SchoolOutlineIcon from "vue-material-design-icons/SchoolOutline.vue";
import BriefcaseAccountIcon from "vue-material-design-icons/BriefcaseAccount.vue";
import BlurOffIcon from "vue-material-design-icons/BlurOff.vue";
import LoadingIcon from "vue-material-design-icons/Loading.vue";
import { useOverlayStore } from "../stores/overlay";

export default {
  components: {
    HomeAccountIcon,
    SchoolOutlineIcon,
    BriefcaseAccountIcon,
    BlurOffIcon,
    LoadingIcon,
  },
  data() {
    return {
      overlayStore: useOverlayStore(),
    };
  },
  methods: {
    handleSelect(e) {
      e.stopPropagation();
      const selection = e.target.value;
      this.overlayStore.selectOverlay(selection);
    },
    handleClick(e) {
      e.stopPropagation();
    },
  },
};
</script>

<style lang="scss" scoped>
</style>
