import {
  countryNameToFlagEmoji,
  eventToEmoji,
  formatJSON,
  getColorOfTag,
  getRecentRecords,
  getResultType,
} from '../helpers/wca-live-helpers.js';
import { getWcalive, updateWcalive } from './db-controller.js';
import { difference, map } from 'ramda';
import { centisecondsToTime } from '../tools/calculator.js';
import { MessageEmbed } from 'discord.js';

const getNewRecords = async () => {
  const recentRecords = await getRecentRecords();

  const formattedRecentRecords = formatJSON(recentRecords.recentRecords);

  const oldRecords = formatJSON(await getWcalive());

  await updateWcalive(recentRecords);

  return difference(formattedRecentRecords, oldRecords);
};

/*
 * Format:
 * Title: {event} {event_emoji} {single/average/mean} of {result}
 * Link: {wca_live_url}
 * Description: {name} from {country} {flag_emoji}
 * Image: Image of WR/CR/NR
 */
const formatRecord = (records) =>
  map(
    (r) =>
      new MessageEmbed()
        .setTitle(
          `${r.result.round.competitionEvent.event.name} ${
            eventToEmoji[r.result.round.competitionEvent.event.id]
          } ${getResultType(
            r.type,
            r.result.round.competitionEvent.event.id
          )} of ${centisecondsToTime(r.attemptResult)}`
        )
        .setURL(
          `https://live.worldcubeassociation.org/competitions/${r.result.round.competitionEvent.competition.id}/rounds/${r.result.round.id}`
        )
        .setDescription(
          `${r.result.person.name} from ${
            r.result.person.country.name
          } ${countryNameToFlagEmoji(r.result.person.country.name)}`
        )
        .setColor(getColorOfTag[r.tag])
        .setThumbnail(
          `https://raw.githubusercontent.com/Nogesma/wca-bot/main/img/${r.tag}.png`
        ),
    records
  );

export { getNewRecords, formatRecord };
