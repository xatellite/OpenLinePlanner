import { defineStore } from "pinia";
import { getMethods, getAdminBounds } from "../helpers/api";

export const useDataStore = () => {
  const innerStore = defineStore({
    id: "data",
    state: () => ({
      methods: null,
      addPoint: null,
      areas: [],
      noAreas: false,
      selectedMethod: null,
      selectedArea: null,
    }),
    getters: {
      questionsResolved: (state) => {
        console.log("check");
        console.log(state.selectedMethod.questions);
        if (state.selectedMethod) {
          for (const question in state.selectedMethod.questions) {
            if (!question.answer) return false;
          }
          return true;
        }
        return false;
      },
    },
    actions: {
      setAddPoint(value) {
        this.addPoint = value;
        this.resetSelection();
        getAdminBounds()
          .then((areas) => {
            this.areas = areas;
            if (areas.length == 0) {
              this.noAreas = true;
            }
          })
          .catch(() => {
            console.log("error");
            this.noAreas = true;
          });
      },
      removeAddPoint() {
        this.addPoint = null;
      },
      selectMethod(method) {
        // ToDo: Fix this workaround..?!
        this.selectedMethod = JSON.parse(JSON.stringify(method));
      },
      selectArea(area) {
        this.selectedArea = area;
      },
      answerQuestion(index, answer) {
        this.selectedMethod.questions[index].answer = answer;
      },
      async loadMethods() {
        this.methods = await getMethods();
      },
      resetSelection() {
        this.selectedArea = null;
        this.selectedMethod = null;
        this.noAreas = false;
        this.areas = [];
      },
    },
  });
  // Async Init
  const s = innerStore();
  s.loadMethods();
  return s;
};
