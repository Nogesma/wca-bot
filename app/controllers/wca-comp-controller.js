import {
  __,
  difference,
  filter,
  includes,
  map,
  prepend,
  propSatisfies,
} from "ramda";
import countryCodeEmoji from "country-code-emoji";
import { EmbedBuilder } from "discord.js";
import { getName } from "country-list";

import { getWcacomp, updateWcacomp } from "./db-controller.js";
import { eventToEmoji } from "../helpers/global-helpers.js";
import {
  getUpcomingCompetitions,
  prettifyTwoDates,
} from "../helpers/wca-comp-helpers.js";

const getNewCompetitions = async () => {
  const upcomingCompetitions = await getUpcomingCompetitions();
  const oldCompetitions = await getWcacomp();

  if (upcomingCompetitions.length) await updateWcacomp(upcomingCompetitions);

  const newComps = oldCompetitions.length
    ? difference(upcomingCompetitions, oldCompetitions)
    : [];

  return filter(
    propSatisfies(
      includes(__, [
        "FR",
        "GB",
        "BE",
        "CH",
        "DE",
        "NL",
        "ES",
        "PT",
        "IT",
        "CA",
        "AD",
        "MC",
        "SM",
        "LU",
        "LI",
      ]),
      "country_iso2"
    ),
    newComps
  );
};

const formatCompetition = map((comp) =>
  comp.cancelled_at === null
    ? {
        embed: new EmbedBuilder()
          .setTitle(`**${comp.name}**`)
          .setURL(comp.url)
          .setThumbnail(
            "https://raw.githubusercontent.com/thewca/worldcubeassociation.org/e974e9020e8c8a1e562c57695b96b312efb8eafa/WcaOnRails/public/files/WCAlogo_50x50.png"
          )
          .setColor("#00FF00")
          .addFields([
            { name: "Ville", value: comp.city, inline: true },
            {
              name: "Pays",
              value: `__**${getName(comp.country_iso2)}**__ ${countryCodeEmoji(
                comp.country_iso2
              )}`,
              inline: true,
            },
            {
              name: "Adresse",
              value: `[${comp.venue_address}](https://duckduckgo.com/?ia=maps&iaxm=maps&q=${comp.latitude_degrees},${comp.longitude_degrees})`,
            },
            {
              name: "Competiteurs max",
              value: String(comp.competitor_limit),
            },
            {
              name: "Date",
              value: prettifyTwoDates(comp.start_date, comp.end_date),
              inline: true,
            },
            {
              name: "Inscriptions",
              value: prettifyTwoDates(
                comp.registration_open,
                comp.registration_close
              ),
              inline: true,
            },
          ]),
        reactions: prepend(
          "<:WCA:862620349376364554>",
          map((id) => eventToEmoji[id], comp.event_ids)
        ),
      }
    : {
        embed: new EmbedBuilder()
          .setTitle(`**${comp.name}**`)
          .setURL(comp.url)
          .setThumbnail(
            "https://raw.githubusercontent.com/thewca/worldcubeassociation.org/e974e9020e8c8a1e562c57695b96b312efb8eafa/WcaOnRails/public/files/WCAlogo_50x50.png"
          )
          .setColor("#FF0000")
          .setDescription("La compétition a été annulée."),
        reactions: ["<:RIP:421349840467787776>"],
      }
);

export { getNewCompetitions, formatCompetition };
