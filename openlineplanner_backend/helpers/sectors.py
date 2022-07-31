import math
from config.statics import MAX_STEPS, ZOOM_FACTOR, MIN_STEPS

def define_sectors(steps, layer_references):
  sectors = {}
  breaker = (90 / steps)

  for reference in layer_references:
    sector_lat = math.floor(reference["lat"] / breaker) * breaker
    sector_lng = math.floor(reference["lng"] / breaker) * breaker

    if sector_lat not in sectors:
      sectors[sector_lat] = {}
      sectors[sector_lat][sector_lng] = {"children":[reference], "stations": []}
    elif sector_lng not in sectors[sector_lat]:
      sectors[sector_lat][sector_lng] = {"children":[reference], "stations": []}
    else:
      sectors[sector_lat][sector_lng]["children"].append(reference)

  return sectors

def define_layers(data_layers):
  point_refs = []
  for data_layer in data_layers:
    data_layer_data = data_layers[data_layer]["data"]
    for point_index, geometry in enumerate(data_layer_data.geometry):
      point_refs.append({"lat": geometry.y, "lng": geometry.x, "index": point_index, "data_layer": data_layer})
  
  steps = MAX_STEPS
  layer_references = point_refs
  layers = {}
  while(steps >= MIN_STEPS and len(layer_references) > 1):
    layer = define_sectors(steps, layer_references)
    layer_references = []
    for lat_sector in layer.keys():
      for lng_sector in layer[lat_sector].keys():
        layer_references.append({"lat": lat_sector, "lng": lng_sector})
    layers[steps] = layer
    steps = steps / ZOOM_FACTOR
  
  for point_ref in point_refs:
    if "base" not in layers:
      layers["base"] = {}
    if point_ref["lat"] not in layers["base"]:
      layers["base"][point_ref["lat"]] = {}
    layers["base"][point_ref["lat"]][point_ref["lng"]] = point_ref

  return layers