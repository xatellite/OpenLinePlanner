
from config.statics import DATA_LAYERS, MAX_STEPS, MIN_STEPS
from helpers.geometry import get_distance

def check_station_relation(parent_layer, layers, stations, steps, data_layer_types, method="closest", decision_distance=500):
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
      # ToDo: This segment can be simplified and speed up
      if len(sector["stations_possible"]) >= 1 or "station_selected" in sector:
        for child_sector_ref in sector["children"]:
          child_sector = layer[child_sector_ref["lat"]][child_sector_ref["lng"]]
          if is_base_layer:
            stations_to_be_checked = sector["stations_possible"]
            if "station_selected" in sector:
              stations_to_be_checked = [sector["station_selected"]]
            if method == "closest":
              # Check closest station and apply pax
              point_data_layer, point_index, closest_station = find_closest_station(child_sector_ref, child_sector, stations_to_be_checked, decision_distance)
              if not point_data_layer == None:
                relations[point_data_layer][point_index] = closest_station
            else:
              # Apply pax to all stations covering point
              point_index = child_sector["index"]
              point_data_layer = child_sector["data_layer"]
              relations[point_data_layer][point_index] = []
              for station in stations_to_be_checked:
                distance = get_distance(station, child_sector_ref)
                if (distance <= decision_distance):
                  relations[point_data_layer][point_index].append({"station": station, "d": distance})
          else:
            if "station_selected" in sector:
              child_sector["station_selected"] = sector["station_selected"]
            else:
              check_sector_station_relation(child_sector_ref["lat"], child_sector_ref["lng"], child_sector, sector["stations_possible"], steps, decision_distance)
  if not is_base_layer:
    return check_station_relation(layer, layers, stations, steps * 10, data_layer_types, method, decision_distance)
  return relations

def find_closest_station(point, sector, stations, decision_distance):
  # calculate for each building
  min_distance = None
  closest_station = None
  for station in stations:
    distance = get_distance(station, point)
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


def calculate_relations(stations, layers, decision_distance, method="closest"):
  top_layer_steps = MIN_STEPS

  # initialize stations
  for sector_lat in layers[top_layer_steps]:
    for sector_lng in layers[top_layer_steps][sector_lat]:
      sector = layers[top_layer_steps][sector_lat][sector_lng]
      sector["stations_possible"] = stations
  return check_station_relation(layers[top_layer_steps], layers, stations, top_layer_steps * 10, list(DATA_LAYERS.keys()), method, decision_distance)