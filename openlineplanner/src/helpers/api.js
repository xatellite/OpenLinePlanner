export const getLayers = async () => {
  return [{}];
};

export const getAdminBounds = async () => {
  return [
    {
      osmId: 23123123,
      name: "Gemeinde Groß-Enzersdorf",
      geojson: {},
    },
    {
      osmId: 23123543,
      name: "Bezirk Groß-Enzersdorf",
      geojson: {},
    },
  ];
};

export const postCalculate = async () => {
  return true;
};

export const getMethods = async () => {
  // const response = await fetch(
  //   import.meta.env.VITE_API_ENDPOINT + "/methods",
  //   {
  //     headers: {
  //       "Content-type": "application/json",
  //     },
  //   }
  // );
  // return await response.json();
  return [
    {
      title: "OpenHousePopulator",
      description: "Using OSM Data and population count",
      questions: [
        {
          id: "population",
          type: "number",
          text: "Total population in this area",
        },
      ],
    },
  ];
};
