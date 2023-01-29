import { CronJob } from "cron";
import { map } from "ramda";

import logger from "../tools/logger.js";
import { formatRecord, getNewRecords } from "./wca-live-controller.js";
import {
  formatCompetition,
  getNewCompetitions,
} from "./wca-comp-controller.js";

const cronList_ = [];

const startCron = (bot) => {
  cronList_.push(
    new CronJob({
      cronTime: "0 0 * * * *",
      onTick: async () => {
        const newRecords = await getNewRecords();

        const formattedRecords = formatRecord(newRecords);

        const chan = await bot.channels.fetch(process.env.WCA_LIVE);

        await Promise.all(
          map((message) => chan.send({ embeds: [message] }), formattedRecords)
        );
      },
      start: false,
      timeZone: "Europe/Paris",
    })
  );

  cronList_.push(
    new CronJob({
      cronTime: "0 0 6 * * *",
      onTick: async () => {
        const newCompetitions = await getNewCompetitions();
        const formattedCompetitions = formatCompetition(newCompetitions);

        const forum = await bot.channels.fetch(process.env.WCA_COMP);

        const threads = await forum.threads.fetch();
        for (const { embed, reactions, country } of formattedCompetitions) {
          const chan = threads.threads.find((x) => x.name === country);

          await chan
            .send({ embeds: [embed] })
            .then((mess) =>
              Promise.all(map((emoji) => mess.react(emoji), reactions))
            );
        }
      },
      start: false,
      timeZone: "Europe/Paris",
    })
  );

  cronList_.forEach((c) => c.start());
  logger.info("Cron Started");
};

const stopCron = () => {
  cronList_.forEach((c) => c.stop());
  logger.info("Cron Stopped");
};

export { startCron, stopCron };
