import math
from helpers.relations import calculate_relations
from config.statics import DATA_LAYERS
from geojson import Feature, Point, FeatureCollection

def calculate_inhabitants_by_stations(stations, method = "absolute", decision_distance = 500, separation_distance = 300):
  """Calculates inhabitants covered by station radius

  Args:
      stations (list): list of stations
      method (str): "absolute" or "relative" [x 1/sqrt(distance)] calculation
      decision_distance (int): max coverage distance of station (default is 500)
      separation_distance (int): distance to split pedestrian and bike transit (default is 300)

  Returns:
      dict: relations sorted by station
  """
  data_layers = DATA_LAYERS
  relations = calculate_relations(stations, data_layers, decision_distance)
  station_relations = {}
  empty_relation = {"ped": 0, "bike": 0, "total": 0, "residential": 0, "work": 0, "school": 0}
  for data_layer in data_layers:
    for relation_index in relations[data_layer]:
      relation = relations[data_layer][relation_index]
      for field in data_layers[data_layer]["fields"]:
        peopleCount = data_layers[data_layer]["data"][field["name"]][relation_index]

        if method == "relative":
          peopleCount = peopleCount * (1 / math.sqr(relation["d"]))
        elif method != "absolute":
          # ToDo: Handle differently
          raise Exception("Method not allowed")
        
        if relation["id"] not in station_relations:
          station_relations[relation["id"]] = empty_relation.copy()
        if relation["d"] < separation_distance:
          station_relations[relation["id"]]["ped"] += peopleCount
        else:
          station_relations[relation["id"]]["bike"] += peopleCount
        station_relations[relation["id"]]["total"] += peopleCount
        station_relations[relation["id"]][field["type"]] += peopleCount
  
  return station_relations

def generate_geojson(stations):
  """Generates geojson including all calculated properties by feature point

  Args:
      stations (list): list of stations
      decision_distance (int): max coverage distance of station (default is 500)

  Returns:
      geojson: of all data points from all data_layers covered by stations
  """
  data_layers = DATA_LAYERS
  relations = calculate_relations(stations, data_layers)

  station_relations = {}
  for data_layer in data_layers:
    for relation_index in relations[data_layer]:
      relation = relations[data_layer][relation_index]
      point_geometry = data_layers[data_layer]["data"].geometry[relation_index]
      point_data = {
        "geometry": point_geometry,
        "properties": {
          "data_layer": data_layer,
          "distance": relation["d"],
          "closest_station": relation["id"]
        }
      }
      if relation["id"] not in station_relations:
        station_relations[relation["id"]] = []
      station_relations[relation["id"]].append(point_data)

  feature_list = []
  for station in stations:
    for point in station_relations[station["id"]]:
      geojson_point = Point((point["geometry"].x, point["geometry"].y))
      feature_list.append(Feature(geometry=geojson_point, properties=point["properties"]))

  return FeatureCollection(feature_list)

