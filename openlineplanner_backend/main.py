import geopandas
# from shapely.geometry import Point
from helpers.geometry import get_distance
import math

gdf = geopandas.read_file("./geo_data/centroids_Groß-Enzersdorf.gpkg", layer='centroids_Groß-Enzersdorf')
max_cycling_distance = 500
max_walking_distance = 300

def calculate_passengers_to_stations(stations):
  buildings_next_station = [ {"distance": None, "station": -1, "pop": pop} for pop in gdf["pop"]]
  buildings_next_station[len(buildings_next_station) -1]["distance"] = 23000
  for building_index, building in enumerate(gdf.geometry):
    for station_index, station in enumerate(stations):
      building_cords = {"lng": building.x, "lat": building.y}
      distance = get_distance(station, building_cords)
      station_distance = buildings_next_station[building_index]
      if station_distance["station"] == -1 or station_distance["distance"] > distance:
        station_distance["station"] = station_index
        station_distance["distance"] = distance

  stations = [ {"lat": station["lat"],"lng": station["lng"], "total": 0, "ped": 0, "bike": 0, "car": 0, "leisure": 0, "residential": 0}  for station in stations]
  # ToDo Check only stations closer 500m - Runtime improvement
  for building in buildings_next_station:
    if building["distance"] and building["distance"] < 500:
      if building["distance"] < 300:
        stations[building["station"]]["ped"] += building["pop"]
      else:
        stations[building["station"]]["bike"] += building["pop"]
      stations[building["station"]]["total"] += building["pop"]
      stations[building["station"]]["residential"] += building["pop"]
  return stations

def find_optimal_station_spot_on_route(stations, route):
  spots_to_check = []
  for point_index in range(1, len(route)):
    point_one = route[point_index - 1]
    point_two = route[point_index]
    distance = get_distance(point_one, point_two)
    steps = int(distance / 10)

    add_lat = (point_two["lat"] - point_one["lat"]) / steps
    add_lng = (point_two["lng"] - point_one["lng"]) / steps
    for step in range(steps):
      spots_to_check.append({"lat": point_one["lat"]+(step*add_lat), "lng":point_one["lng"]+(step*add_lng), "pax": 0})
  
  print(len(spots_to_check))
  
  for building_index, building in enumerate(gdf.geometry):
    # Skip if building is already covered by other station:
    skip = False;
    for station in stations:
      building_cords = {"lng": building.x, "lat": building.y}
      distance = get_distance(station, building_cords)
      if distance < max_cycling_distance:
        skip = True
        break
    if skip == True: continue
    # Check all spots the building could access:
    # ToDo: Add weighting here if needed.
    for spot in spots_to_check:
      building_cords = {"lng": building.x, "lat": building.y}
      distance = get_distance(spot, building_cords)
      
      # Check building only if in range
      if distance > len(spots_to_check) * 10 + 500:
        break

      if distance < max_cycling_distance :
        spot["pax"] += gdf["pop"][building_index] * (1/math.sqrt(distance))
  
  spots_to_check.sort(key=lambda spot: -spot["pax"])
  return spots_to_check[0]


# station1 = {"lng": 16.637762089587426, "lat": 48.188910876924076}
# station2 = {"lng": 16.63747946839206, "lat": 48.189507272900414}
# stations = [station1, station2]
# print(calculate_passengers_to_stations(stations))