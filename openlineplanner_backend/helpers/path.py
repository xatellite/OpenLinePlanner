from helpers.geometry import get_distance

def split_path_into_points(route, coverage):
  spots_to_check = []
  for point_index in range(1, len(route)):
    point_one = route[point_index - 1]
    point_two = route[point_index]
    distance = get_distance(point_one, point_two)
    steps = int(distance / 10)

    if (steps <= 0): continue

    add_lat = (point_two["lat"] - point_one["lat"]) / steps
    add_lng = (point_two["lng"] - point_one["lng"]) / steps

    for step in range(0, steps):
      spots_to_check.append({
        "lat": point_one["lat"]+(step*add_lat),
        "lng": point_one["lng"]+(step*add_lng),
        "coverage": coverage,
        "id": str(point_index)+"_"+str(step),
        "index": point_index
      })
  return spots_to_check