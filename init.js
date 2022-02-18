import { Amplify } from "aws-amplify";
import * as fs from "fs";

import {
  updateWcalive,
  updateWcacomp,
} from "./app/controllers/db-controller.js";
import { getRecentRecords } from "./app/helpers/wca-live-helpers.js";
import { getUpcomingCompetitions } from "./app/helpers/wca-comp-helpers.js";

if (fs.existsSync("./data")) process.exit(0);

Amplify.configure({
  API: {
    graphql_endpoint: "https://live.worldcubeassociation.org/api/graphql",
  },
});

fs.mkdirSync("./data");

const records = await getRecentRecords();
updateWcalive(records);

const upcomingCompetitions = await getUpcomingCompetitions();
updateWcacomp(upcomingCompetitions);
