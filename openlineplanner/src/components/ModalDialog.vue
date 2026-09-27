<template>
  <div v-if="shown" class="modal" @click.self="close">
    <div class="modal__container">
      <div class="modal__head">
        <span class="modal__head__title">{{ title }}</span>
        <button @click="close" class="modal__head__close button--transparent">
          <CloseIcon />
        </button>
      </div>
      <div class="modal__content">
        <slot></slot>
      </div>
    </div>
  </div>
</template>

<script>
import CloseIcon from "vue-material-design-icons/Close.vue";

export default {
  components: { CloseIcon },
  props: {
    shown: Boolean,
    title: String,
    // Callback prop rather than an emit, matching the convention used by
    // TooltipButton and TextInput.
    onClose: Function,
  },
  methods: {
    close() {
      if (this.onClose) this.onClose();
    },
  },
};
</script>

<style lang="scss" scoped>
.modal {
  position: fixed;
  top: 0;
  left: 0;
  z-index: 10;
  width: 100vw;
  height: 100vh;
  background-color: rgba(0, 0, 0, 0.3);
  display: flex;
  align-items: center;
  justify-content: center;

  &__container {
    background-color: var(--c-background-primary);
    border-radius: $br-md;
    box-shadow: var(--bs-md);
    max-width: 520px;
    width: 90%;
    max-height: 85vh;
    padding: $space-sm;
    display: flex;
    flex-direction: column;
    box-sizing: border-box;
  }

  &__head {
    display: flex;
    justify-content: space-between;
    align-items: center;

    &__title {
      font-size: $font-lg;
      font-weight: bold;
    }

    &__close {
      width: auto;
    }
  }

  &__content {
    display: flex;
    flex-direction: column;
    overflow-y: auto;
    padding: $space-sm 0 0;
  }
}
</style>
