import {
  countryNameToFlagEmoji,
  eventToEmoji,
  formatJSON,
  getRecentRecords,
  tagToEmoji,
} from '../helpers/wca-live-helpers.js';
import { getWcalive, updateWcalive } from './db-controller.js';
import { difference, map } from 'ramda';
import { centisecondsToTime } from '../tools/calculator.js';

const getNewRecords = async () => {
  const newRecords = await getRecentRecords();

  const recentRecords = formatJSON(newRecords.recentRecords);

  const oldRecords = formatJSON(await getWcalive());

  await updateWcalive(newRecords);

  return difference(recentRecords, oldRecords);
};
/*
 * Format: {name} ({country} {flag_emoji}) a battu le {WR/CR/NR} {single/average}
 *         de {event} {event_emoji} avec un score de {result} ({competition})
 */

const formatRecord = (records) =>
  map(
    (r) =>
      `${r.result.person.name} (${
        r.result.person.country.name
      } ${countryNameToFlagEmoji(r.result.person.country.name)}) a battu le ${
        tagToEmoji[r.tag]
      } ${r.type} de ${r.result.round.competitionEvent.event.name} ${
        eventToEmoji[r.result.round.competitionEvent.event.id]
      } avec un score de **${centisecondsToTime(r.attemptResult)}** (${
        r.result.round.competitionEvent.competition.name
      })`,
    records
  );

export { getNewRecords, formatRecord };
