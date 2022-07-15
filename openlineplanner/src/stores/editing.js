import { defineStore } from "pinia";

export const useEditStore = defineStore({
  id: "edit",
  state: () => ({
    isEditing: null, // line_ref
    isExtending: null, // station index in line 0 / -1
    isMerging: null, // Station ref
    pointSelected: null,
  }),
  actions: {
    stopAllInputs() {
      this.isEditing = null;
      this.isExtending = null;
      this.isMerging = null;
      this.pointSelected = null;
    },
  },
});
