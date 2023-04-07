export const getLayers = async () => {
  const response = await fetch(import.meta.env.VITE_API_ENDPOINT + "/layer");
  const layerInfo = await response.json();
  return layerInfo;
};

export const getBBoxForLayer = async (layer) => {
};

export const getAdminBounds = async (point) => {
  const response = await fetch(import.meta.env.VITE_API_ENDPOINT + `/osm/admin_bounds/${point.lat}/${point.lng}`);
  return await response.json();
};

export const postCalculate = async () => {
  return true;
};

export const getMethods = async () => {
  const response = await fetch(import.meta.env.VITE_API_ENDPOINT + "/layer/methods");
  const methodList = await response.json();
  return methodList;
};
