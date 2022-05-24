export const selectFile = () => {
  const fileChooser = parent.settings.document.createElement("input");
  fileChooser.type = "file";
  fileChooser.click();
  return fileChooser.files[0];
};

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
