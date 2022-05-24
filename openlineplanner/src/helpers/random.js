export const randomInteger = (max) => {
  return Math.floor(Math.random() * (max + 1));
};

export const randomRgbColor = () => {
  let r = randomInteger(255);
  let g = randomInteger(255);
  let b = randomInteger(255);
  return [r, g, b];
};

export const randomColor = () => {
  let [r, g, b] = randomRgbColor();

  let hr = r.toString(16).padStart(2, "0");
  let hg = g.toString(16).padStart(2, "0");
  let hb = b.toString(16).padStart(2, "0");

  return "#" + hr + hg + hb;
};
