// Massive ToDo: Add error handling for all of these endpoints

export const getLayers = async () => {
  const response = await fetch(import.meta.env.VITE_API_ENDPOINT + "/layer");
  const layerInfo = await response.json();
  return layerInfo;
};

export const getLayer = async (layerId) => {
  const response = await fetch(
    import.meta.env.VITE_API_ENDPOINT + "/layer/" + layerId
  );
  const layerInfo = await response.json();
  return layerInfo;
};

export const getBBoxForLayer = async (layer) => {};

export const getAdminBounds = async (point) => {
  const response = await fetch(
    import.meta.env.VITE_API_ENDPOINT +
      `/osm/admin_bounds/${point.lat}/${point.lng}`
  );
  return await response.json();
};

export const postCalculate = async (method, area) => {
  const response = await fetch(
    import.meta.env.VITE_API_ENDPOINT + "/layer/calculate",
    {
      method: "POST",
      body: JSON.stringify({
        layer_type: method.layer_type,
        method: method.method,
        answers: method.questions.map((question) => ({
          name: question.id,
          value: question.answer,
        })),
        area: area,
        name: area.properties.name,
      }),
      headers: {
        "Content-Type": "application/json",
      },
    }
  );
  await response.json();
  return true;
};

export const getMethods = async () => {
  const response = await fetch(
    import.meta.env.VITE_API_ENDPOINT + "/layer/methods"
  );
  const methodList = await response.json();
  return methodList;
};

export const deleteLayer = async (layer) => {
  await fetch(import.meta.env.VITE_API_ENDPOINT + `/layer/${layer.id}`, {
    method: "DELETE",
  });
  return;
};

export const getMapCenter = async () => {
  const response = await fetch(
    import.meta.env.VITE_API_ENDPOINT + "/layer/center"
  );
  const mapCenter = await response.json();
  return mapCenter;
};

export const getStreetAddressName = async (point) => {
  const data = await fetch(
    "https://nominatim.openstreetmap.org/reverse.php?lat=" +
      point.lat +
      "&lon=" +
      point.lng +
      "&zoom=18&format=jsonv2",
    {
      method: "GET",
    }
  );
  const geocodingResult = await data.json();
  return geocodingResult.address.road;
};
