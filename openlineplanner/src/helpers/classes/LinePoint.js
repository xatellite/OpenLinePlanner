export default class LinePoint {
  constructor(
    lat,
    lng,
    line,
    type = "line",
    id = crypto.randomUUID(),
    lines = [line]
  ) {
    this.id = id;
    this.lat = lat;
    this.lng = lng;
    this.type = type;
    this.lines = lines;
  }

  static fromObject(pointObject) {
    return new LinePoint(
      pointObject.lat,
      pointObject.lng,
      pointObject.line,
      pointObject.type,
      pointObject.id,
      pointObject.lines
    );
  }

  updatePosition(position) {
    this.lat = position.lat;
    this.lng = position.lng;
  }
}
