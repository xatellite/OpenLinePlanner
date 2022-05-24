import LinePoint from "./LinePoints";

export default class TransportLine {
  constructor(id, name) {
    this.id = id;
    this.name = name;
    this.points = [];
    // this.halfwayPoints = [];
    this.type = "tram";
    this.color = "#ff0000";
  }

  toLineString() {
    return {
      type: "FeatureCollection",
      features: [
        {
          type: "Feature",
          geometry: {
            type: "LineString",
            coordinates: this.points.map((point) => [point.lng, point.lat]),
          },
        },
      ],
    };
  }

  getPointById(id) {
    return this.points.find((point) => point.id == id);
  }

  getLineLongId() {
    return `line-${this.id}`;
  }

  setName(name) {
    this.name = name;
  }

  calculateHalfwayPoints() {
    // ToDo: CalcultateHalfway points
  }

  addPoint(lng, lat, index=-1) {
    const newPoint = new LinePoint(lng, lat, this);
    // ToDo Add point at position:
    if (index != -1) {
      this.points.splice(index, 0, newPoint);
    } else {
      this.points = [...this.points, newPoint];
    }
    this.calculateHalfwayPoints();
    return newPoint.id;
  }
}
