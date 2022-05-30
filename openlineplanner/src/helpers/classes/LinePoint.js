export default class LinePoint {
  constructor(
    lat,
    lng,
    line,
    type = "line",
    id = crypto.randomUUID(),
    lines = [line],
    name = ""
  ) {
    this.id = id;
    this.lat = lat;
    this.lng = lng;
    this.type = type;
    this.lines = lines;
    this.name = name;
  }

  static fromObject(pointObject) {
    return new LinePoint(
      pointObject.lat,
      pointObject.lng,
      pointObject.line,
      pointObject.type,
      pointObject.id,
      pointObject.lines,
      pointObject.name
    );
  }

  copy() {
    const copied = Object.assign(
      Object.create(Object.getPrototypeOf(this)),
      this
    );
    return copied;
  }

  updatePosition(position) {
    this.lat = position.lat;
    this.lng = position.lng;
  }
}
