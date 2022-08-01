from config.statics import DATA_LAYERS

def data_layer_to_geo_json(data_layer_name):
  """Generate geojson of data_layer

  Args:
      data_layer_name (string): name of data_layer

  Returns:
      geojson: valid representation of data_layer
  """
  return DATA_LAYERS[data_layer_name]["data"].to_json()