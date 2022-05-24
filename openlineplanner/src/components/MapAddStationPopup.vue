<template>
  <div class="box add-station">
    <button @click="addStation"><PlusIcon /></button>
    <span>Add Station</span>
  </div>
</template>

<script>
import PlusIcon from "vue-material-design-icons/Plus.vue";
export default {
  props: {
    point: Object,
  },
  components: {
    PlusIcon,
  },
  methods: {
    addStation() {
      this.point.type = "station";
    },
  },
};
</script>

<style lang="scss">
@import "@/assets/variables.scss";
.add-station {
  position: absolute;
  top: -100px;
  display: flex;
}
</style>