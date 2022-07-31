from helpers.geodata import load_gdf

MAX_STEPS = 500000
ZOOM_FACTOR = 10
MIN_STEPS = 50

# Asset min steps
# assert((MIN_STEPS / (MAX_STEPS % ZOOM_FACTOR))%1 == 0)
# decision distance must be larger than smallest step


gdf_residence = load_gdf("./geo_data/residence")
gdf_school = load_gdf("./geo_data/schools")
gdf_jobs = load_gdf("./geo_data/jobs")

DATA_LAYERS = {
    "residence": {
      "data": gdf_residence,
      "fields": [{
        "name": "pop",
        "type": "residential"
      }]
    },
    "jobs": {
      "data": gdf_jobs,
      "fields": [{
        "name": "jobs",
        "type": "work"
      }]
    },
    "school": {
      "data": gdf_school,
      "fields": [{
        "name": "teachers",
        "type": "work"
      },
      {
        "name": "kids",
        "type": "school"
      }]
    },
  }