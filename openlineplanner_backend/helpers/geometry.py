import math

def get_distance(point_one, point_two):
  lng_1 = point_one["lng"]
  lat_1 = point_one["lat"]
  lng_2 = point_two["lng"]
  lat_2 = point_two["lat"]
  lng_1, lat_1, lng_2, lat_2 = map(math.radians, [lng_1, lat_1, lng_2, lat_2])
  d_lat = lat_2 - lat_1
  d_lng = lng_2 - lng_1 

  temp = (  
      math.sin(d_lat / 2) ** 2 
    + math.cos(lat_1) 
    * math.cos(lat_2) 
    * math.sin(d_lng / 2) ** 2
  )

  return int(6373.0 * (2 * math.atan2(math.sqrt(temp), math.sqrt(1 - temp))) * 1000)