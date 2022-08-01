from processing import find_optimal_station_spot_on_route
from snapshottest import TestCase
import unittest

class OptimalStationTest(TestCase):

    def test_small(self):
      point1 = {"lng": 16.444110412596956, "lat": 48.23368923042784, "id": "point1"}
      point2 = {"lng": 16.48839904785234, "lat": 48.22040872909554, "id": "point2"}
      route = [point1, point2]

      station1 = {"lng": 16.64321174146274, "lat": 48.19013354182042, "id": "station1"}
      stations = [station1]

      actualResult_improved = find_optimal_station_spot_on_route(stations, route)
      self.assertMatchSnapshot(actualResult_improved)

      actualResult_improved = find_optimal_station_spot_on_route(stations, route, "relative")
      self.assertMatchSnapshot(actualResult_improved)


if __name__ == '__main__':
    unittest.main()
