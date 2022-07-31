import falcon
from falcon.media.validators import jsonschema
from main import data_layer_to_geo_json #, find_optimal_station_spot_on_route
from schemas import passenger_resource_schema, overlay_resource_schema
import json
import numpy as np

from processing import calculate_inhabitants_by_stations
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
    @jsonschema.validate(passenger_resource_schema)
    def on_post(self, req, resp):
        obj = req.get_media()

        # ToDo Sanity check
        stations = obj.get('stations')
       
        station_info = calculate_inhabitants_by_stations(stations)
        station_info = json.loads(json.dumps(station_info, cls=NpEncoder))

        resp.media = { "stationInfo": station_info}

# class FinderResource:
#     def on_post(self, req, resp):
#         obj = req.get_media()

#         # ToDo Sanity check
#         stations = obj.get('stations')
#         route = obj.get('route')
       
#         station_info = find_optimal_station_spot_on_route(stations, route)

#         resp.media = { "optimalStation": station_info};
class OverlayResource:
    @jsonschema.validate(overlay_resource_schema)
    def on_post(self, req, resp):
        obj = req.get_media()

        # ToDo Sanity check
        layer_name = obj.get('layer')
        layer_data = data_layer_to_geo_json(layer_name)
        resp.media = { "layerGeoJson": layer_data}


application = app = falcon.App(middleware=falcon.CORSMiddleware(
    allow_origins='*',
    allow_credentials='*',
))
app.add_route('/station-info', PassengerResource())
# app.add_route('/find-station', FinderResource())
app.add_route('/overlay', OverlayResource())
