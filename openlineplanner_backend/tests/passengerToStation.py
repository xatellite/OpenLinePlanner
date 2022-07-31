from processing import calculate_inhabitants_by_stations
from snapshottest import TestCase
import unittest

# TESTING: Only for profiling purposes
# import cProfile
# from pstats import Stats
# import line_profiler
# profiler = line_profiler.LineProfiler()


class CalculateStationTest(TestCase):

    # Setup cProfiler
    def setUp(self):
      # """init each test"""
      # self.pr = cProfile.Profile()
      # self.pr.enable()
      # profiler.add_function(calculate_passengers_to_stations)
      pass

    
    def tearDown(self):
        """finish any test"""
        # p = Stats(self.pr)
        # p.strip_dirs()
        # p.sort_stats ('cumtime')
        # p.print_stats ()
        # profiler.dump_stats('foo.lprof')
        # lstats = profiler.get_stats()
        # line_profiler.show_text(lstats.timings, lstats.unit)
        pass


    # @profiler
    def test_small(self):
      station1 = {"lng": 16.444110412596956, "lat": 48.23368923042784, "id": "station1"}
      station2 = {"lng": 16.48839904785234, "lat": 48.22040872909554, "id": "station2"}
      stations = [station1, station2]

      actualResult_improved = calculate_inhabitants_by_stations(stations)
      self.assertMatchSnapshot(actualResult_improved) 

    def test_big(self):
      station1 = {"lng": 16.444110412596956, "lat": 48.23368923042784, "id": "station1"}
      station2 = {"lng": 16.46865798950239, "lat": 48.23127294374399, "id": "station2"}
      station3 = {"lng": 16.488399047851715, "lat": 48.22498341634133, "id": "station3"}
      station4 = {"lng": 16.498527069091807, "lat": 48.21995123788474, "id": "station4"}
      station5 = {"lng": 16.503505249023334, "lat": 48.21308837909456, "id": "station5"}
      station6 = {"lng": 16.533889312744833, "lat": 48.21148691300917, "id": "station6"}
      station7 = {"lng": 16.545047302246644, "lat": 48.218121232155255, "id": "station7"}
      station8 = {"lng": 16.549757690431193, "lat": 48.203144752675286, "id": "station8"}
      stations = [station1, station2,station3,station4,station5, station6, station7, station8]
      
      actualResult_improved = calculate_inhabitants_by_stations(stations)
      self.assertMatchSnapshot(actualResult_improved) 


if __name__ == '__main__':
    unittest.main()
