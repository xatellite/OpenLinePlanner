<template>
  <div id="color-picker" class="box color-picker">
    <ColorPicker
      theme="light"
      :color="color"
      :colors-default="colorsDefault"
      :sucker-hide="true"
      :sucker-canvas="suckerCanvas"
      :sucker-area="suckerArea"
      @changeColor="changeColor"
    />
  </div>
</template>

<script>
import { ColorPicker } from "vue-color-kit";
import "vue-color-kit/dist/vue-color-kit.css";

export default {
  props: {
    handleColorChange: Function,
    closeAction: Function,
    initColor: String,
  },
  components: {
    ColorPicker,
  },
  data() {
    return {
      color: this.initColor,
      suckerCanvas: null,
      suckerArea: [],
      isSucking: false,
      colorsDefault: ["#fec80b","#cb2727", "#00a651", "#ed1e26", "#996c3c", "#f48220", "#a066ab", "#0091d8", "#840f18", "#141845"],
    };
  },
  mounted() {
    window.addEventListener("click", this.handleClickOutside);
  },
  methods: {
    changeColor(color) {
      const { r, g, b, a } = color.rgba;
      this.color = `rgba(${r}, ${g}, ${b}, ${a})`;
      this.handleColorChange(color.hex);
    },
    handleClickOutside(e) {
      console.log(e);

      let refElement = e.target;
      while (refElement.parentElement != null) {
        if (refElement.id === "color-picker") {
          return;
        }
        refElement = refElement.parentElement;
      }
      this.closeAction();
    },
  },
  beforeUnmount() {
    window.removeEventListener("click", this.handleClickOutside);
  },
};
</script>

<style lang="scss" scoped>
.color-picker {
  position: absolute;
  top: 0;
  left: 100%;
  z-index: 100;
}
</style>
