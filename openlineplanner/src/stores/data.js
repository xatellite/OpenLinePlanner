import { defineStore } from "pinia";
import {
  getMethods,
  getAdminBounds,
  getLayers,
  deleteLayer,
  postCalculate,
} from "../helpers/api";

export const useDataStore = () => {
  const innerStore = defineStore({
    id: "data",
    state: () => ({
      methods: null,
      addPoint: null,
      layers: [],
      areas: [],
      noAreas: false,
      selectedMethod: null,
      selectedArea: null,
      mapBounds: [],
      mapHighligh: null,
      loadingLayer: false,
    }),
    getters: {
      questionsResolved(state) {
        if (state.selectedMethod) {
          for (const index in state.selectedMethod.questions) {
            const question = state.selectedMethod.questions[index];
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
        getAdminBounds(value)
          .then((areas) => {
            this.areas = areas.features;
            this.removeMapBoundsByType("proposal");
            this.areas.forEach((area) => {
              this.addMapBound(area, "area", "proposal", "#1B998B");
            });
            this.mapHighligh = null;
            if (areas.length == 0) {
              this.noAreas = true;
            }
          })
          .catch((err) => {
            console.log("error", err);
            this.noAreas = true;
          });
      },
      removeAddPoint() {
        this.addPoint = null;
      },
      addMapBound(area, type, ref, color) {
        console.log(area);
        area.properties.type = type;
        area.properties.color = color;
        area.properties.ref = ref;
        this.mapBounds = [...this.mapBounds, area];
      },
      removeMapBoundsById(id) {
        this.mapBounds = this.mapBounds.filter(
          (area) => area.properties.id != id
        );
      },
      removeMapBoundsByType(ref) {
        this.mapBounds = this.mapBounds.filter(
          (area) => area.properties.ref != ref
        );
      },
      highlightArea(area) {
        this.mapHighligh = area;
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
      async loadLayers() {
        this.layers = await getLayers();
      },
      calculateLayer() {
        this.loadingLayer = true;
        postCalculate(this.selectedMethod, this.selectedArea)
          .then(() => {
            this.loadingLayer = false;
            this.resetSelection();
            this.loadLayers();
          })
          .catch(() => {
            this.loadingLayer = false;
            this.resetSelection();
            this.loadLayers();
          });
      },
      async removeLayer(layer) {
        console.log(layer);
        this.removeMapBoundsById(layer.id);
        await deleteLayer(layer);
        this.loadLayers();
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
  s.loadLayers();
  return s;
};
