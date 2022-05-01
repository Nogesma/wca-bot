import discord, { Intents } from "discord.js";
import logger from "./app/tools/logger.js";

import { startCron, stopCron } from "./app/controllers/cron-controller.js";

const bot = new discord.Client({
  intents: [
    Intents.FLAGS.GUILD_MESSAGES,
    Intents.FLAGS.GUILDS,
    Intents.FLAGS.GUILD_EMOJIS_AND_STICKERS,
    Intents.FLAGS.GUILD_MESSAGE_REACTIONS,
  ],
});

bot.on("ready", () => {
  logger.info("Bot ready");
  startCron(bot);
  bot.user.setActivity({ name: "for new records", type: 3 });
});

bot.login(process.env.TOKEN);

process.on("exit", () => {
  stopCron();
  logger.info("Exiting");
});
