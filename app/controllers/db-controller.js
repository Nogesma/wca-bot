import * as fs from "fs";

const updateWcalive = (doc) =>
  fs.writeFile("./data/wcalive.json", JSON.stringify(doc), "utf8", () => {});

const getWcalive = () =>
  JSON.parse(fs.readFileSync("./data/wcalive.json", "utf8"));

const updateWcacomp = (doc) =>
  fs.writeFile("./data/wcacomp.json", JSON.stringify(doc), "utf8", () => {});

const getWcacomp = () =>
  JSON.parse(fs.readFileSync("./data/wcacomp.json", "utf8"));

export { updateWcalive, getWcalive, updateWcacomp, getWcacomp };
