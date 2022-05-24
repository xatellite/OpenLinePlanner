import falcon
from main import calculate_passengers_to_stations, find_optimal_station_spot_on_route

class PassengerResource:
    def on_post(self, req, resp):
        obj = req.get_media()

        # ToDo Sanity check
        stations = obj.get('stations')
       
        station_info = calculate_passengers_to_stations(stations)

        resp.media = { "stationInfo": station_info};

class StopFinder:
    def on_post(self, req, resp):
        obj = req.get_media()

        # ToDo Sanity check
        stations = obj.get('stations')
        route = obj.get('route')
       
        station_info = find_optimal_station_spot_on_route(stations, route)

        resp.media = { "optimalStation": station_info};

app = falcon.App()
app.add_route('/station-info', PassengerResource())
app.add_route('/find-station', StopFinder())
