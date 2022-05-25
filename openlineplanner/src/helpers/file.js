export const selectFile = (callback) => {
  const fileChooser = document.createElement("input");
  fileChooser.type = "file";
  fileChooser.click();
  fileChooser.onchange = () => {
    callback(fileChooser.files[0]);
  };
};

export const readJSONFile = (file, callback) => {
  const reader = new FileReader();
  reader.onload = () => {
    callback(JSON.parse(reader.result));
  };
  reader.readAsText(file);
}

export const downloadJSON = (jsonData, filename = "Lines.save") => {
  let body = document.body;
  const a = document.createElement("a");
  a.href = URL.createObjectURL(
    new Blob([JSON.stringify(jsonData, null, 2)], {
      type: "text/plain",
    })
  );
  a.setAttribute("download", filename);
  body.appendChild(a);
  a.click();
  body.removeChild(a);
};
