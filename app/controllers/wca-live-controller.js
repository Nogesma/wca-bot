import { difference, map } from "ramda";
import { EmbedBuilder } from "discord.js";

import {
  formatAttemptResult,
  getColorOfTag,
  getRecentRecords,
  getResultType,
} from "../helpers/wca-live-helpers.js";
import { getWcalive, updateWcalive } from "./db-controller.js";
import { eventToEmoji } from "../helpers/global-helpers.js";
import countryCodeEmoji from "country-code-emoji";

const getNewRecords = async () => {
  const recentRecords = await getRecentRecords();
  const oldRecords = await getWcalive();

  if (recentRecords.length) await updateWcalive(recentRecords);

  return oldRecords.length ? difference(recentRecords, oldRecords) : [];
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
      new EmbedBuilder()
        .setTitle(
          `${r.result.round.competitionEvent.event.name} ${
            eventToEmoji[r.result.round.competitionEvent.event.id]
          } ${getResultType(
            r.type,
            r.result.round.competitionEvent.event.id,
          )} of ${formatAttemptResult({
            attemptResult: r.attemptResult,
            eventId: r.result.round.competitionEvent.event.id,
          })}`,
        )
        .setURL(
          `https://live.worldcubeassociation.org/competitions/${r.result.round.competitionEvent.competition.id}/rounds/${r.result.round.id}`,
        )
        .setDescription(
          `${r.result.person.name} from ${
            r.result.person.country.name
          } ${countryCodeEmoji(r.result.person.country.iso2)}`,
        )
        .setColor(getColorOfTag[r.tag])
        .setThumbnail(
          `https://raw.githubusercontent.com/Nogesma/wca-bot/main/img/${r.tag}.png`,
        ),
    records,
  );

export { getNewRecords, formatRecord };
