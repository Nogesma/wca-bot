import discord from 'discord.js';
import logger from './app/tools/logger.js';
import mongoose from 'mongoose';

import { startCron, stopCron } from './app/controllers/cron-controller.js';

mongoose.connect(process.env.MONGO_URL || 'mongodb://localhost:27017/wca', {
  useNewUrlParser: true,
  useFindAndModify: false,
  useCreateIndex: true,
  useUnifiedTopology: true,
});

const bot = new discord.Client();

bot.on('ready', () => {
  logger.info('Bot ready');
  startCron(bot);
  bot.user.setPresence({
    activity: { name: 'for new records', type: 3 },
  });
});

bot.login(process.env.TOKEN);

process.on('exit', () => {
  stopCron();
  mongoose.disconnect();
  logger.info('Exiting');
});

// import Parser from 'rss-parser';
//
// const parser = new Parser();
//
// (async () => {
//   let feed = await parser.parseURL('https://www.worldcubeassociation.org/rss');
//   console.log(feed.title);
//
//   feed.items.forEach((item) => {
//     if (includes('france', item.title))
//       console.log(item.title + ':' + item.link);
//   });
// })();
