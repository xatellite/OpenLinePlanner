<template>
  <div class="action-toolbar">
    <div class="action-toolbar__standard">
      <button @click="save"><TrayArrowDownIcon /></button>
      <button @click="load"><FolderUploadOutlineIcon /></button>
    </div>
    <button
      class="action-toolbar__stop"
      v-if="editStore.isEditing"
      @click="disableEditing"
    >
      <PencilOffOutlineIcon />
    </button>
  </div>
</template>

<script>
import TrayArrowDownIcon from "vue-material-design-icons/TrayArrowDown.vue";
import FolderUploadOutlineIcon from "vue-material-design-icons/FolderUploadOutline.vue";
import PencilOffOutlineIcon from "vue-material-design-icons/PencilOffOutline.vue";
import { useEditStore } from "../stores/editing";
import { useLinesStore } from "../stores/lines";
import { selectFile, downloadJSON } from "../helpers/file";

export default {
  components: {
    TrayArrowDownIcon,
    FolderUploadOutlineIcon,
    PencilOffOutlineIcon,
  },
  setup() {
    const editStore = useEditStore();
    const linesStore = useLinesStore();

    return {
      editStore,
      linesStore,
    };
  },
  methods: {
    disableEditing() {
      this.editStore.isEditing = null;
      this.editStore.isAdding = null;
      this.editStore.isExtending = null;
    },
    save() {
      const editStore = this.editStore.$state;
      const linesStore = this.linesStore.$state;
      // const filename = selectFile();
      downloadJSON({ editStore, linesStore });
    },
    load() {
      // Upload
      // resolve
      // fetch("test.json")
      //   .then(response => response.json())
      //   .then(json => console.log(json));
    },
  },
};
</script>

<style lang="scss">
@import "@/assets/variables.scss";
.action-toolbar {
  display: flex;
  align-items: center;
  padding: $space-ssm;
  justify-content: space-between;

  * > {
    margin: $space-ssm;
  }

  &__standard {
    display: flex;
  }

  &__stop {
    color: red;
  }
}
</style>
