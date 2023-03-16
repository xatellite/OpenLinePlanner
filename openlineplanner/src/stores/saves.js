import { defineStore } from "pinia";
import { useEditStore } from '@/stores/editing';
import { useLinesStore } from '@/stores/lines';
import { downloadJSON } from "../helpers/file";

const loadSaves = () => {
  return JSON.parse(localStorage.saves || null) || [];
};

export const useSavesStore = defineStore({
  id: "saves",
  state: () => ({
    saves: loadSaves(),
  }),
  actions: {
    save(project) {
      // Workaround to deep clone object
      this.saves.push(JSON.parse(JSON.stringify(project)));
      localStorage.saves = JSON.stringify(this.saves);
    },
    load(index) {
      useEditStore().stopAllInputs();
      console.log(this.saves[index]);
      useLinesStore().loadState(this.saves[index]);
    },
    download(index) {
      downloadJSON({ lineStore: this.saves[index] }, `${this.saves[index].projectName}.save`);
    },
    remove(index) {
      this.saves.splice(index, 1);
      localStorage.saves = JSON.stringify(this.saves);
    },
  },
});
