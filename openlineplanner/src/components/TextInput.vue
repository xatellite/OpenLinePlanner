<template>
  <div class="text-input">
    <span class="text-input__title">{{ title }}</span>
    <input
      class="text-input__field"
      :type="type"
      :value="value"
      :placeholder="placeholder"
      @input="(e) => callback(e.target.value)"
    />
    <PencilOutlineIcon />
  </div>
</template>

<script>
import PencilOutlineIcon from "vue-material-design-icons/PencilOutline.vue";

export default {
  props: {
    callback: Function,
    value: String,
    title: String,
    placeholder: String,
    type: {
      type: String,
      default: "text",
    }
  },
  components: {
    PencilOutlineIcon,
  },
};
</script>

<style lang="scss" scoped>
.text-input {
  position: relative;
  background-color: $c-box;
  display: block;
  padding: $space-ssm $space-md;
  border: 1px solid $c-button-border;
  border-radius: $br-md;
  display: flex;
  margin-top: $space-sm;

  &:focus-within {
    outline: 1px solid $c-accent-two;

    .text-input__title {
      color: $c-accent-two;
    }
  }

  &__title {
    position: absolute;
    top: -0.8em;
    color: $c-text-primary;
    background-color: $c-box;
    left: $space-sm;
    padding: 0 $space-ssm;
  }

  &__field {
    border: none;
    flex-grow: 1;
    font-size: $font-md;
    min-width: 200px;
    margin-right: $space-sm;

    &:focus {
      outline: none;
    }
  }
}
</style>
