import falcon
from falcon.media.validators import jsonschema
from main import data_layer_to_geo_json
from schemas import passenger_resource_schema, overlay_resource_schema, optimal_station_schema
import json
import numpy as np

from processing import calculate_inhabitants_by_stations, find_optimal_station_spot_on_route
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
        # Build arg object
        args = {}
        args["stations"] = obj.get('stations')
        if obj.get('method'):
            args["method"] = obj.get('method')
        if obj.get('decision_distance'):
            args["decision_distance"] = obj.get('decision_distance')
        if obj.get('separation_distance'):
            args["separation_distance"] = obj.get('separation_distance')
        # Retrieve result
        station_info = calculate_inhabitants_by_stations(**args)
        station_info = json.loads(json.dumps(station_info, cls=NpEncoder))
        resp.media = { "stationInfo": station_info}

class FinderResource:
    @jsonschema.validate(optimal_station_schema)
    def on_post(self, req, resp):
        obj = req.get_media()
        # Build arg object
        args = {}
        args["stations"] = obj.get('stations')
        args["route"] = obj.get('route')
        if obj.get('method'):
            args["method"] = obj.get('method')
        if obj.get('decision_distance'):
            args["decision_distance"] = obj.get('decision_distance')
        # Retrieve result
        station_info = find_optimal_station_spot_on_route(**args)
        resp.media = { "optimalStation": station_info}

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
app.add_route('/find-station', FinderResource())
app.add_route('/overlay', OverlayResource())
