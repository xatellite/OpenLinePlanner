import LineTypeParameters from "../../assets/defaults/lineTypes.json";

export default class TransportLine {
  constructor(
    name,
    id = crypto.randomUUID(),
    type = "tram",
    color = "#ff0000",
    pointIds = [],
    customAcceleration = 1.0,
    customStopTime = 30,
    customMaxSpeed = 50,
    customCoverage = 300,
    lineThickness = 2.0
  ) {
    this.id = id;
    this.name = name;
    this.type = type;
    this.color = color;
    this.pointIds = pointIds;
    this.customAcceleration = customAcceleration;
    this.customStopTime = customStopTime;
    this.lineThickness = lineThickness;
    this.customCoverage = customCoverage;
    this.customMaxSpeed = customMaxSpeed;
  }

  static fromObject(lineObject) {
    return new TransportLine(
      lineObject.name,
      lineObject.id,
      lineObject.type,
      lineObject.color,
      lineObject.pointIds,
      lineObject.customAcceleration,
      lineObject.customStopTime,
      lineObject.lineThickness,
      lineObject.customCoverage,
      lineObject.customMaxSpeed
    );
  }

  getLineLongId() {
    return `line-${this.id}`;
  }

  getCoverage() {
    if (this.type === "custom") {
      return this.customCoverage;
    }
    return LineTypeParameters[this.type]["coverageArea"];
  }

  getLineThickness() {
    if (this.type === "custom") {
      return this.lineThickness;
    }
    return LineTypeParameters[this.type]["lineWidth"];
  }

  setName(name) {
    this.name = name;
  }

  addPoint(pointId, index = -1) {
    if (index != -1) {
      this.pointIds.splice(index, 0, pointId);
    } else {
      this.pointIds = [...this.pointIds, pointId];
    }
    return pointId;
  }
}
