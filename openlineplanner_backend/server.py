import falcon
from main import calculate_passengers_to_stations, find_optimal_station_spot_on_route
import json
import numpy as np

class NpEncoder(json.JSONEncoder):
    def default(self, obj):
        if isinstance(obj, np.integer):
            return int(obj)
        if isinstance(obj, np.floating):
            return float(obj)
        if isinstance(obj, np.ndarray):
            return obj.tolist()
        return json.JSONEncoder.default(self, obj)


class PassengerResource:
    def on_post(self, req, resp):
        obj = req.get_media()

        # ToDo Sanity check
        stations = obj.get('stations')
       
        station_info = calculate_passengers_to_stations(stations)
        station_info = json.loads(json.dumps(station_info, cls=NpEncoder))

        resp.media = { "stationInfo": station_info};

class StopFinder:
    def on_post(self, req, resp):
        obj = req.get_media()

        # ToDo Sanity check
        stations = obj.get('stations')
        route = obj.get('route')
       
        station_info = find_optimal_station_spot_on_route(stations, route)

        resp.media = { "optimalStation": station_info};

app = falcon.App(middleware=falcon.CORSMiddleware(
    allow_origins='*',
    allow_credentials='*',
))
app.add_route('/station-info', PassengerResource())
app.add_route('/find-station', StopFinder())
