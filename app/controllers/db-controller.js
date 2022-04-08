import * as fs from "fs";

const updateWcalive = (doc) =>
  fs.writeFile("./data/wcalive.json", JSON.stringify(doc), "utf8", () => {});

const getWcalive = () => {
  const wcalive = fs.readFileSync("./data/wcalive.json", "utf8");

  if (wcalive.length) return JSON.parse(wcalive);
  return [];
};

const updateWcacomp = (doc) =>
  fs.writeFile("./data/wcacomp.json", JSON.stringify(doc), "utf8", () => {});

const getWcacomp = () => {
  const wcacomp = fs.readFileSync("./data/wcacomp.json", "utf8");

  if (wcacomp.length) return JSON.parse(wcacomp);
  return [];
};

export { updateWcalive, getWcalive, updateWcacomp, getWcacomp };
