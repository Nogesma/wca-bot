import { CronJob } from 'cron';
import { forEach } from 'ramda';

import logger from '../tools/logger.js';
import { formatRecord, getNewRecords } from './wca-live-controller.js';
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

        forEach(
          (message) => chan.send({ embeds: [message] }),
          formattedRecords
        );
      },
      start: false,
      timeZone: 'Europe/Paris',
    })
  );

  cronList_.push(
    new CronJob({
      cronTime: '0 0 6 * * *',
      onTick: async () => {
        const newCompetitions = await getNewCompetitions();
        const formattedCompetitions = formatCompetition(newCompetitions);

        const chan = await bot.channels.fetch(process.env.WCA_COMP);

        forEach(({ embed, reactions }) => {
          chan
            .send({ embeds: [embed] })
            .then((mess) => forEach((emoji) => mess.react(emoji), reactions));
        }, formattedCompetitions);
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
