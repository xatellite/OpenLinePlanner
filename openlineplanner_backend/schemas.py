station_list_schema = {
  "type": "array",
  "items": {
    "type": "object",
    "properties": {
      "id" : { "type": "string" },
      "lat" : { "type": "number" },
      "lng" : { "type": "number" }
    },
    "required":["lat", "lng", "id"]
  },
  "minItems": 0,
  "uniqueItems": True,
}

point_list_schema = {
  "type": "array",
  "items": {
    "type": "object",
    "properties": {
      "lat" : { "type": "number" },
      "lng" : { "type": "number" }
    },
    "required":["lat", "lng"]
  },
  "minItems": 2,
  "uniqueItems": True,
}

station_resource_schema = {
 "type":"object",
    "properties": {
        "stations": station_list_schema,
        "decision_distance": { "type" : "integer" },
        "separation_distance": { "type" : "integer" },
        "method": { "type" : "string" },
      },
  "required":["stations"]
}

coverage_resource_schema = {
  "type":"object",
  "properties": {
      "stations": station_list_schema,
      "decision_distance": { "type" : "integer" },
    },
  "required":["stations"]
}

overlay_resource_schema = {
 "type":"object",
    "properties": {
        "layer_name": { "type" : "string" },
      },
  "required":["layer_name"]
}

optimal_station_schema = {
  "type":"object",
    "properties": {
        "stations": station_list_schema,
        "route": point_list_schema,
        "decision_distance": { "type" : "integer" },
        "method": { "type" : "string" },
      },
  "required":["stations", "route"]
}