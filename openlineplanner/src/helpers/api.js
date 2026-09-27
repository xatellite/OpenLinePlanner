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

// Region data import
// These back the setup modal, which lets a user load OSM data for a new area
// instead of preparing it by hand on the server.

export const getRegionCatalog = async () => {
  const response = await fetch(
    import.meta.env.VITE_API_ENDPOINT + "/data/catalog"
  );
  if (!response.ok) throw new Error(await response.text());
  return await response.json();
};

export const getImportedRegions = async () => {
  const response = await fetch(
    import.meta.env.VITE_API_ENDPOINT + "/data/regions"
  );
  if (!response.ok) throw new Error(await response.text());
  return await response.json();
};

// Previews which extract would be downloaded, so the user sees the size before
// committing to a download that can run into hundreds of megabytes.
export const postResolveExtract = async (area, catalogId) => {
  const response = await fetch(
    import.meta.env.VITE_API_ENDPOINT + "/data/resolve",
    {
      method: "POST",
      body: JSON.stringify({ area, catalog_id: catalogId ?? null }),
      headers: { "Content-Type": "application/json" },
    }
  );
  if (!response.ok) throw new Error(await response.text());
  return await response.json();
};

export const postDataImport = async (area, catalogId) => {
  const response = await fetch(
    import.meta.env.VITE_API_ENDPOINT + "/data/import",
    {
      method: "POST",
      body: JSON.stringify({ area, catalog_id: catalogId ?? null }),
      headers: { "Content-Type": "application/json" },
    }
  );
  if (!response.ok) throw new Error(await response.text());
  return await response.json();
};

export const getImportJob = async (jobId) => {
  const response = await fetch(
    import.meta.env.VITE_API_ENDPOINT + `/data/import/${jobId}`
  );
  if (!response.ok) throw new Error(await response.text());
  return await response.json();
};

export const searchPlace = async (query) => {
  const response = await fetch(
    "https://nominatim.openstreetmap.org/search?format=jsonv2&limit=5&q=" +
      encodeURIComponent(query)
  );
  if (!response.ok) throw new Error(await response.text());
  return await response.json();
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
