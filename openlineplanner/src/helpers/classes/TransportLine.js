import LineTypeParameters from "../../assets/defaults/lineTypes.json";

export default class TransportLine {
  constructor(
    name,
    id = crypto.randomUUID(),
    type = "tram",
    color = "#ff0000",
    pointIds = [],
    customSpeedLimits = {},
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
    this.customSpeedLimits = customSpeedLimits;
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
      lineObject.customSpeedLimits,
      lineObject.customAcceleration,
      lineObject.customStopTime,
      lineObject.customMaxSpeed,
      lineObject.customCoverage,
      lineObject.lineThickness
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

  getMaxSpeed() {
    if (this.type === "custom") {
      return this.customMaxSpeed;
    }
    return LineTypeParameters[this.type]["maxSpeed"];
  }

  getStopTime() {
    if (this.type === "custom") {
      return this.customStopTime;
    }
    return LineTypeParameters[this.type]["stopTime"];
  }

  getAcceleration() {
    if (this.type === "custom") {
      return this.customAcceleration;
    }
    return LineTypeParameters[this.type]["acceleration"];
  }

  setName(name) {
    this.name = name;
  }

  getSectionSpeed(stationId) {
    if (Object.keys(this.customSpeedLimits).includes(stationId)) {
      return this.customSpeedLimits[stationId];
    }
    return this.getMaxSpeed();
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
