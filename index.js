import discord, { Intents } from 'discord.js';
import logger from './app/tools/logger.js';
import mongoose from 'mongoose';

import { startCron, stopCron } from './app/controllers/cron-controller.js';
import { incomingMessage } from './app/controllers/message-controller.js';
import { Amplify } from 'aws-amplify';

mongoose.connect(process.env.MONGO_URL || 'mongodb://localhost:27017/wca');

Amplify.configure({
  API: {
    graphql_endpoint: 'https://live.worldcubeassociation.org/api/graphql',
  },
});

const bot = new discord.Client({
  intents: [
    Intents.FLAGS.GUILD_MESSAGES,
    Intents.FLAGS.GUILDS,
    Intents.FLAGS.GUILD_EMOJIS_AND_STICKERS,
  ],
});

bot.on('ready', () => {
  logger.info('Bot ready');
  startCron(bot);
  bot.user.setActivity({ name: 'for new records', type: 3 });
});

bot.on('messageCreate', incomingMessage);

bot.login(process.env.TOKEN);

process.on('exit', () => {
  stopCron();
  mongoose.disconnect();
  logger.info('Exiting');
});
