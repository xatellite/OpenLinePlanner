import math
from helpers.relations import calculate_relations
from config.statics import DATA_LAYERS
from geojson import Feature, Point, FeatureCollection
import copy
from helpers.sectors import define_layers
from helpers.path import split_path_into_points

main_layers = define_layers(DATA_LAYERS)

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
  layers = copy.deepcopy(main_layers)
  relations = calculate_relations(stations, layers, decision_distance)
  station_relations = {}
  empty_relation = {"ped": 0, "bike": 0, "total": 0, "residential": 0, "work": 0, "school": 0}
  for data_layer in data_layers:
    for relation_index in relations[data_layer]:
      relation = relations[data_layer][relation_index]
      for field in data_layers[data_layer]["fields"]:
        peopleCount = data_layers[data_layer]["data"][field["name"]][relation_index]

        if method == "relative":
          if (relation["d"] <= 0):
            peopleCount = peopleCount
          else:
            peopleCount = peopleCount * (1 / math.sqrt(relation["d"]))
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
  # Add station wich have not been mentioned
  for station in stations:
    if station["id"] not in station_relations:
      station_relations[station["id"]] = empty_relation
  return station_relations

def generate_geojson(stations):
  """Generates geojson including all calculated properties by feature point

  Args:
      stations (list): list of stations

  Returns:
      geojson: of all data points from all data_layers covered by stations
  """
  layers = copy.deepcopy(main_layers) 
  data_layers = DATA_LAYERS
  relations = calculate_relations(stations, layers, data_layers)

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

def find_optimal_station_spot_on_route(stations, route, method = "absolute", decision_distance = 500):
  """Finds the optimal spot to place a new station on a route

  Args:
      stations (list): list of stations
      route (list): list of all points forming the rout the new station should be placed on
      method (string): weighting method of point search: "absolute" or "relative" [x 1/sqrt(distance)] calculation
      decision_distance (int): max coverage distance of station (default is 500)

  Returns:
      object: point of new optimal station
  """
  layers = copy.deepcopy(main_layers) 
  modified_data_layers = copy.deepcopy(DATA_LAYERS)
  relations = calculate_relations(stations, layers, decision_distance)

  for data_layer in DATA_LAYERS:
    modified_data_layers[data_layer]["data"] = modified_data_layers[data_layer]["data"].drop(relations[data_layer].keys())
    modified_data_layers[data_layer]["data"] = modified_data_layers[data_layer]["data"].reset_index()

  updated_data_layers = define_layers(modified_data_layers)
  points_to_check = split_path_into_points(route)

  relations = calculate_relations(points_to_check, updated_data_layers, decision_distance, method="all")

  # Calculate pax
  station_relations = {}
  empty_relation = {"total": 0, "residential": 0, "work": 0, "school": 0}
  for data_layer in modified_data_layers:
    for relation_index in relations[data_layer]:
      for relation_station in relations[data_layer][relation_index]:
        for field in modified_data_layers[data_layer]["fields"]:
          peopleCount = modified_data_layers[data_layer]["data"][field["name"]][relation_index]

          if method == "relative":
            peopleCount = peopleCount * (1 / math.sqrt(relation_station["d"]))
          elif method != "absolute":
            # ToDo: Handle differently
            raise Exception("Method not allowed")
          
          station_id = relation_station["station"]["id"]
          if station_id not in station_relations:
            station_relations[station_id] = empty_relation.copy()
          
          station_relations[station_id]["geometry"] = relation_station["station"]
          station_relations[station_id]["total"] += peopleCount
          station_relations[station_id][field["type"]] += peopleCount

  # Select point with highest pax
  max_total = None
  max_total_point = None
  for station_list_id in station_relations:
    station = station_relations[station_list_id]
    if max_total == None or station["total"] > max_total:
      max_total = station["total"]
      max_total_point = station["geometry"]
  return max_total_point
