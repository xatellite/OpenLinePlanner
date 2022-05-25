export default class TransportLine {
  constructor(id, name, type = "tram", color = "#ff0000", pointIds = []) {
    this.id = id;
    this.name = name;
    this.pointIds = pointIds;
    this.type = type;
    this.color = color;
  }

  static fromObject(lineObject) {
    return new TransportLine(
      lineObject.id,
      lineObject.name,
      lineObject.type,
      lineObject.color,
      lineObject.pointIds
    );
  }

  getLineLongId() {
    return `line-${this.id}`;
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
