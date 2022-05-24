import { defineStore } from "pinia";

export const useEditStore = defineStore({
  id: "edit",
  state: () => ({
    isEditing: null, // line_ref
    isAdding: null, // line_ref
    isExtending: null, // line_ref

    pointSelected: null,
  }),
});
