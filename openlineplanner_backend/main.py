from helpers.geometry import get_distance
import math
from config.statics import DATA_LAYERS

def data_layer_to_geo_json(data_layer_name):
  """Generate geojson of data_layer

  Args:
      data_layer_name (string): name of data_layer

  Returns:
      geojson: valid representation of data_layer
  """
  return DATA_LAYERS[data_layer_name].to_json()


# ToDo: To be reworked entirely
# def find_optimal_station_spot_on_route(stations, route):
#   spots_to_check = []
#   for point_index in range(1, len(route)):
#     point_one = route[point_index - 1]
#     point_two = route[point_index]
#     distance = get_distance(point_one, point_two)
#     steps = int(distance / 10)

#     add_lat = (point_two["lat"] - point_one["lat"]) / steps
#     add_lng = (point_two["lng"] - point_one["lng"]) / steps
#     for step in range(steps):
#       spots_to_check.append({"lat": point_one["lat"]+(step*add_lat), "lng":point_one["lng"]+(step*add_lng), "pax": 0})
  
#   for building_index, building in enumerate(gdf_residence.geometry):
#     # Skip if building is already covered by other station:
#     skip = False;
#     for station in stations:
#       building_cords = {"lng": building.x, "lat": building.y}
#       distance = get_distance(station, building_cords)
#       if distance < max_cycling_distance:
#         skip = True
#         break
#     if skip == True: continue
#     # Check all spots the building could access:
#     # ToDo: Add weighting here if needed.
#     for spot in spots_to_check:
#       building_cords = {"lng": building.x, "lat": building.y}
#       distance = get_distance(spot, building_cords)
      
#       # Check building only if in range
#       if distance > len(spots_to_check) * 10 + 500:
#         break

#       if distance < max_cycling_distance:
#         if distance > 0:
#           spot["pax"] += gdf_residence["pop"][building_index] * (1/math.sqrt(distance))
#         else:
#           spot["pax"] += gdf_residence["pop"][building_index]
  
#   spots_to_check.sort(key=lambda spot: -spot["pax"])
#   return spots_to_check[0]
