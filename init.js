import * as fs from "fs";

import {
  updateWcalive,
  updateWcacomp,
} from "./app/controllers/db-controller.js";
import { getRecentRecords } from "./app/helpers/wca-live-helpers.js";
import {
  getAllThreads,
  getUpcomingCompetitions,
} from "./app/helpers/wca-comp-helpers.js";
import { map } from "ramda";
import { Client, GatewayIntentBits } from "discord.js";
import logger from "./app/tools/logger.js";
import { COUNTRIES } from "./app/config.js";
import { formatCompetition } from "./app/controllers/wca-comp-controller.js";
import countryCodeEmoji from "country-code-emoji";
import { getCountryName } from "./app/helpers/global-helpers.js";

const initDir = async () => {
  if (!fs.existsSync("./data")) fs.mkdirSync("./data");
  if (!fs.existsSync("./data/wcalive.json")) {
    const records = await getRecentRecords();
    updateWcalive(records);
  }

  const upcomingCompetitions = await getUpcomingCompetitions();
  if (!fs.existsSync("./data/wcacomp.json"))
    updateWcacomp(upcomingCompetitions);

  return upcomingCompetitions;
};
const initForum = async (upcomingCompetitions) => {
  const forum = await bot.channels.fetch(process.env.WCA_COMP);
  const threads = await getAllThreads(forum);

  for (const country of COUNTRIES) {
    const name = `${countryCodeEmoji(country)} ${getCountryName.of(country)}`;
    const chan = threads.find((x) => x.name === name);

    if (chan) continue;

    const thread = await forum.threads.create({
      name,
      message: {
        content: `Compétitions officielles: **${getCountryName.of(country)}**`,
      },
    });

    await Promise.all(
      map(
        ({ embed, reactions }) =>
          thread
            .send({ embeds: [embed] })
            .then((mess) =>
              Promise.all(map((emoji) => mess.react(emoji), reactions)),
            ),
        formatCompetition(
          upcomingCompetitions.filter((x) => x.country_iso2 === country),
        ),
      ),
    );
  }
};

const bot = new Client({
  intents: [
    GatewayIntentBits.Guilds,
    GatewayIntentBits.GuildMessages,
    GatewayIntentBits.GuildEmojisAndStickers,
    GatewayIntentBits.GuildMessageReactions,
  ],
});

bot.on("ready", async () => {
  logger.info("Init ready");
  const comps = await initDir();
  await initForum(comps);
  process.exit();
});

bot.login(process.env.TOKEN);
