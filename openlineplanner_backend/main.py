import geopandas
# from shapely.geometry import Point
from helpers.geometry import get_distance
import math
import os


max_cycling_distance = 500
max_walking_distance = 300

# Load and append available residence Data
def load_gdf(path):
  files = os.listdir(path)
  gdf = geopandas.read_file(path + "/" + files[0], layer= files[0][:-5])
  for file_index in range(1, len(files)):
    gdf_new = geopandas.read_file(path + "/" + files[file_index], layer=files[file_index][:-5])
    gdf.append(gdf_new)
  return gdf


gdf_residence = load_gdf("./geo_data/residence")
gdf_school = load_gdf("./geo_data/schools")
# ToDo: Extend jobs to include leisure + rename to businesses
gdf_jobs = load_gdf("./geo_data/jobs")


def calculate_passengers_to_stations(stations):
  # evaluate central station + distance to furthest neighbor
  distances = [ 0 for x in range(len(stations))]
  furthest_neighbor = [ 0 for x in range(len(stations))]
  for index, station in enumerate(stations):
    for neighbor_station in stations:
      distance_to_neighbor = get_distance(station, neighbor_station)
      distances[index] += distance_to_neighbor
      if (furthest_neighbor[index] < distance_to_neighbor):
        furthest_neighbor[index] = distance_to_neighbor
  central_station_index = distances.index(min(distances))
  central_station_distance = furthest_neighbor[central_station_index]
  

  # get buildings in range
  buildings_next_station = []
  buildings_next_station_indexes = []
  for building_index, building in enumerate(gdf_residence.geometry):
    building_cords = {"lng": building.x, "lat": building.y}
    distance = get_distance(stations[central_station_index], building_cords)
    if (distance < central_station_distance + max_cycling_distance):
      buildings_next_station.append({"distance": None, "station": -1, "pop": gdf_residence["pop"][building_index]})
      buildings_next_station_indexes.append(building_index)


  def evaluate_distances_to_station(geometry, objects, indexes = None):
    if indexes == None:
      indexes = [x for x in range(len(geometry))]
    for object_index, geometry_index in enumerate(indexes):
      object_geometry = geometry[geometry_index]
      for station_index, station in enumerate(stations):
        object_cords = {"lng": object_geometry.x, "lat": object_geometry.y}
        distance = get_distance(station, object_cords)
        station_distance = objects[object_index]
        if station_distance["station"] == -1 or station_distance["distance"] > distance:
          station_distance["station"] = station_index
          station_distance["distance"] = distance

  
  # evaluate distance to stations
  evaluate_distances_to_station(gdf_residence.geometry, buildings_next_station, buildings_next_station_indexes)
  job_next_station = [{"distance": None, "station": -1, "jobs": jobs} for jobs in gdf_jobs["jobs"]]
  evaluate_distances_to_station(gdf_jobs.geometry, job_next_station)

  school_next_station = []
  for kids_index, kids in enumerate(gdf_school["kids"]):
    school_next_station.append({"distance": None, "station": -1, "kids": kids, "teachers": gdf_school["teachers"][kids_index]})
  evaluate_distances_to_station(gdf_school.geometry, school_next_station)

  # write out station values
  stations = [ {"id": station["id"], "lat": station["lat"],"lng": station["lng"], "total": 0, "ped": 0, "bike": 0, "car": 0, "leisure": 0, "residential": 0, "school": 0, "work": 0}  for station in stations]
  for building in buildings_next_station:
    if building["distance"] and building["distance"] < 500:
      if building["distance"] < 300:
        stations[building["station"]]["ped"] += building["pop"]
      else:
        stations[building["station"]]["bike"] += building["pop"]
      stations[building["station"]]["total"] += building["pop"]
      stations[building["station"]]["residential"] += building["pop"]

  for school in school_next_station:
    if school["distance"] and school["distance"] < 500:
      if school["distance"] < 300:
        stations[school["station"]]["ped"] += school["kids"]
        stations[school["station"]]["ped"] += school["teachers"]
      else:
        stations[school["station"]]["bike"] += school["kids"]
        stations[school["station"]]["bike"] += school["teachers"]
      stations[school["station"]]["total"] += school["kids"]
      stations[school["station"]]["total"] += school["teachers"]
      stations[school["station"]]["school"] += school["kids"]
      stations[school["station"]]["work"] += school["teachers"]

  for business in job_next_station:
    if business["distance"] and business["distance"] < 500:
      if business["distance"] < 300:
        stations[business["station"]]["ped"] += business["jobs"]
      else:
        stations[business["station"]]["bike"] += business["jobs"]
      stations[business["station"]]["total"] += business["jobs"]
      stations[business["station"]]["work"] += business["jobs"]

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
  
  for building_index, building in enumerate(gdf_residence.geometry):
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
        spot["pax"] += gdf_residence["pop"][building_index] * (1/math.sqrt(distance))
  
  spots_to_check.sort(key=lambda spot: -spot["pax"])
  return spots_to_check[0]


# station1 = {"lng": 16.637762089587426, "lat": 48.188910876924076}
# station2 = {"lng": 16.63747946839206, "lat": 48.189507272900414}
# stations = [station1, station2]
# print(calculate_passengers_to_stations(stations))