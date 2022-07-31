
from config.statics import MAX_STEPS, MIN_STEPS
from helpers.geometry import get_distance
from helpers.sectors import define_layers

def check_station_relation(parent_layer, layers, stations, steps, data_layer_types, decision_distance=500):
  is_base_layer = False
  relations = {}
  if steps > MAX_STEPS:
    layer = layers["base"]
    is_base_layer = True
    for data_layer_type in data_layer_types:
      relations[data_layer_type] = {}
  else:
    layer = layers[steps]
  for sector_lat in parent_layer:
    for sector_lng in parent_layer[sector_lat]:
      sector = parent_layer[sector_lat][sector_lng]
      if len(sector["stations_possible"]) >= 1 and not "station_selected" in sector:
        for child_sector_ref in sector["children"]:
          child_sector = layer[child_sector_ref["lat"]][child_sector_ref["lng"]]
          if is_base_layer:
            point_layer, point_index, closest_station = find_closest_station(child_sector_ref["lat"], child_sector_ref["lng"], child_sector, sector["stations_possible"], decision_distance)
            if not point_layer == None:
              relations[point_layer][point_index] = closest_station
          else:
            check_sector_station_relation(child_sector_ref["lat"], child_sector_ref["lng"], child_sector, sector["stations_possible"], steps, decision_distance)
      elif "station_selected" in sector:
        for child_sector_ref in sector["children"]:
          child_sector = layer[child_sector_ref["lat"]][child_sector_ref["lng"]]
          if is_base_layer:
            point_layer, point_index, closest_station = find_closest_station(child_sector_ref["lat"], child_sector_ref["lng"], child_sector, [sector["station_selected"]], decision_distance)
            if not point_layer == None:
                relations[point_layer][point_index] = closest_station
          else:
            child_sector["station_selected"] = sector["station_selected"]
  if not is_base_layer:
    return check_station_relation(layer, layers, stations, steps * 10, data_layer_types)
  return relations

def find_closest_station(lat, lng, sector, stations, decision_distance):
  # calculate for each building
  min_distance = None
  closest_station = None
  for station in stations:
    distance = get_distance(station, {"lat": lat, "lng": lng})
    if (min_distance == None or min_distance > distance) and distance <= decision_distance:
      min_distance = distance
      closest_station = station
  if closest_station == None:
    return None, None, None
  return sector["data_layer"], sector["index"], {"id": closest_station["id"], "d": distance}

# inplace
def check_sector_station_relation(lat, lng, sector, stations, steps, decision_distance):
  grid_size = (90 / steps)
  sector_corners = [
    {"lat": lat, "lng": lng},
    {"lat": lat + grid_size, "lng": lng},
    {"lat": lat, "lng": lng + grid_size},
    {"lat": lat + grid_size, "lng": lng + grid_size}
  ]
  for station in stations:
    station_added = False

    # Check all edges are covered and apply station to all children if zoom level under decision width
    if len(sector["stations_possible"]) == 1 and grid_size < 0.004:
      station_select = True
      for sector_corner in sector_corners:
        distance = get_distance(station, sector_corner)
        if (distance <= decision_distance):
          station_added = True
        if (distance > decision_distance):
          station_select = False
          # break if there is no chance for selection and station already reaches section
          if station_added: break
      if station_select:
        sector["station_selected"] = station
        return
      if station_added:
        sector["stations_possible"].append(station)
        return
    else:
      # Check one corner is covered
      for sector_corner in sector_corners:
        if (get_distance(station, sector_corner) <= decision_distance):
          sector["stations_possible"].append(station)
          station_added = True
          break

    # Check point fully inside
    if (
      not station_added and
      station["lat"] > lat and station["lat"] < lat + grid_size
      and station["lng"] > lng and station["lng"] < lng + grid_size
    ):
      sector["stations_possible"].append(station)


def calculate_relations(stations, data_layers, decision_distance):
  layers = define_layers(data_layers)
  top_layer_steps = MIN_STEPS

  # initialize stations
  for sector_lat in layers[top_layer_steps]:
    for sector_lng in layers[top_layer_steps][sector_lat]:
      sector = layers[top_layer_steps][sector_lat][sector_lng]
      sector["stations_possible"] = stations
  return check_station_relation(layers[top_layer_steps], layers, stations, top_layer_steps * 10, list(data_layers.keys()), decision_distance)