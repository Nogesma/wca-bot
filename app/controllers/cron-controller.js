import { CronJob } from 'cron';

import logger from '../tools/logger.js';
import { formatRecord, getNewRecords } from './wca-live-controller.js';
import { forEach } from 'ramda';

const cronList_ = [];

const startCron = (bot) => {
  cronList_.push(
    new CronJob({
      cronTime: '0 0 * * * *',
      onTick: async () => {
        const newRecords = await getNewRecords();

        const formattedRecords = formatRecord(newRecords);

        const chan = await bot.channels.fetch(process.env.WCA_LIVE);

        forEach((message) => chan.send(message), formattedRecords);
      },
      start: false,
      timeZone: 'Europe/Paris',
    })
  );

  cronList_.forEach((c) => c.start());
  logger.info('Cron Started');
};

const stopCron = () => {
  cronList_.forEach((c) => c.stop());
  logger.info('Cron Stopped');
};

export { startCron, stopCron };
