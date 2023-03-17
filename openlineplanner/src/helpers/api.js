export const getLayers = () => {};

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
          type: "number",
          question: "Total population in this area",
          id: "population",
        },
      ],
    },
  ];
};
