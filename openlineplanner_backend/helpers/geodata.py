import geopandas
import os

# Load and append available residence Data
def load_gdf(path):
  files = os.listdir(path)
  gdf = geopandas.read_file(path + "/" + files[0], layer= files[0][:-5])
  for file_index in range(1, len(files)):
    gdf_new = geopandas.read_file(path + "/" + files[file_index], layer=files[file_index][:-5])
    gdf = gdf.append(gdf_new, ignore_index=True)
  return gdf