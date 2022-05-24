export default class LinePoints {
  constructor(lat, lng, line) {
    this.id = crypto.randomUUID();
    this.lat = lat;
    this.lng = lng;
    this.type = "line"; // station, line
    this.lines = [line];
  }

  updatePosition(position) {
    this.lat = position.lat;
    this.lng = position.lng;
  }
}
