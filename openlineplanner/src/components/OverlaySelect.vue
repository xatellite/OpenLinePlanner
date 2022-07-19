<template>
  <div class="overlay-select" @click="handleClick">
    <span>
      <LoadingIcon class="loader" v-if="overlayStore.overlay != 'none' && !overlayStore.overlayData" />
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
@import "@/assets/variables.scss";
.overlay-select {
  @extend button;
  display: flex;
  align-items: center;
  justify-content: center;
  height: 2.5rem;
  width: auto;
  padding: $space-ssm $space-ssm;
  margin: 0;

  & > span {
    border-right: 1px solid #d6d6d6;
    padding: $space-ssm;
    & > span {
      display: flex;
      align-items: center;
      height: 1rem;
      padding: 0;
    }
  }
}

.loader {
  -webkit-animation: infiniteRotate 2s linear infinite;
  animation: infiniteRotate 2s linear infinite;
  height: 1rem;
  padding: 0;
}

/* Safari 4.0 - 8.0 */
@-webkit-keyframes infiniteRotate {
  0% {
    -webkit-transform: rotate(0deg);
  }
  100% {
    -webkit-transform: rotate(360deg);
  }
}
/* Standard syntax */
@keyframes infinite-rotate {
  0% {
    -webkit-transform: rotate(0deg);
  }
  100% {
    -webkit-transform: rotate(360deg);
  }
}

select {
  /* Reset Select */
  appearance: none;
  outline: 0;
  border: 0;
  box-shadow: none;
  /* Personalize */
  flex: 1;
  font-size: $font-md;
  padding: 0 1em;
  color: $c-text-primary;
  background-color: darken($c-box, 2);
  background-image: none;
  cursor: pointer;
}
/* Remove IE arrow */
select::-ms-expand {
  display: none;
}
/* Custom Select wrapper */
.select {
  position: relative;
  display: flex;
  width: 20em;
  height: 3em;
  border-radius: 0.25em;
  overflow: hidden;
}
/* Arrow */
.select::after {
  content: "\25BC";
  position: absolute;
  top: 0;
  right: 0;
  padding: 1em;
  background-color: #34495e;
  transition: 0.25s all ease;
  pointer-events: none;
}
/* Transition */
.select:hover::after {
  color: #f39c12;
}
</style>
