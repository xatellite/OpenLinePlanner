station_list_schema = {
  "type": "array",
  "items": {
    "type": "object",
    "properties": {
      "id" : { "type": "string" },
      "lat" : { "type": "number" },
      "lng" : { "type": "number" }
    },
  },
  "minItems": 0,
  "uniqueItems": True,
}

passenger_resource_schema = {
 "type":"object",
    "properties": {
        "stations": station_list_schema,
        "decision_distance": { "type" : "integer" },
        "separation_distance": { "type" : "integer" },
        "method": { "type" : "string" },
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