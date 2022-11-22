import { Client, GatewayIntentBits } from "discord.js";
import logger from "./app/tools/logger.js";

import { startCron, stopCron } from "./app/controllers/cron-controller.js";

const bot = new Client({
  intents: [
    GatewayIntentBits.Guilds,
    GatewayIntentBits.GuildMessages,
    GatewayIntentBits.GuildEmojisAndStickers,
    GatewayIntentBits.GuildMessageReactions,
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
