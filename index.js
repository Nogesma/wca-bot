import discord from 'discord.js';
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

const bot = new discord.Client();

bot.on('ready', () => {
  logger.info('Bot ready');
  startCron(bot);
  bot.user.setPresence({
    activity: { name: 'for new records', type: 3 },
  });
});

bot.on('message', incomingMessage);

bot.login(process.env.TOKEN);

process.on('exit', () => {
  stopCron();
  mongoose.disconnect();
  logger.info('Exiting');
});
