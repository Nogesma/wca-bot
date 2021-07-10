import { CronJob } from 'cron';

import logger from '../tools/logger.js';
import { formatRecord, getNewRecords } from './wca-live-controller.js';
import { forEach, map } from 'ramda';
import {
  formatCompetition,
  getNewCompetitions,
} from './wca-comp-controller.js';

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

  cronList_.push(
    new CronJob({
      cronTime: '40 30 14 * * *',
      onTick: async () => {
        const newCompetitions = await getNewCompetitions();
        console.log(newCompetitions);
        // const formattedCompetitions = formatCompetition(newCompetitions);
        // console.log(formattedCompetitions);
        //
        // const chan = await bot.channels.fetch(process.env.WCA_LIVE);
        //
        // forEach(({ embed, reactions }) => {
        //   chan
        //     .send(embed)
        //     .then((mess) => map((emoji) => mess.react(emoji), reactions));
        // }, formattedCompetitions);
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
