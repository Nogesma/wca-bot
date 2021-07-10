import {
  eventToEmoji,
  formatJSON,
  getUpcomingCompetitions,
  prettifyTwoDates,
} from '../helpers/wca-live-helpers.js';
import { getWcacomp, updateWcacomp } from './db-controller.js';
import { difference, map } from 'ramda';
import countryCodeEmoji from 'country-code-emoji';
import * as Discord from 'discord.js';
import { getName } from 'country-list';
import * as fs from 'fs';

const getNewCompetitions = async () => {
  const upcomingCompetitions = formatJSON(await getUpcomingCompetitions());

  const oldCompetitions = formatJSON(await getWcacomp()).competitions;

  await updateWcacomp(upcomingCompetitions);
  fs.writeFile('new.json', JSON.stringify(upcomingCompetitions), () => {});
  fs.writeFile('old.json', JSON.stringify(oldCompetitions), () => {});
  console.log(difference(upcomingCompetitions, oldCompetitions));
  return difference(upcomingCompetitions, oldCompetitions);
};

const formatCompetition = map((comp) =>
  comp.cancelled_at === null
    ? {
        embed: new Discord.MessageEmbed()
          .setTitle(`**${comp.name}**`)
          .setURL(comp.url)
          .setThumbnail(
            'https://raw.githubusercontent.com/thewca/worldcubeassociation.org/e974e9020e8c8a1e562c57695b96b312efb8eafa/WcaOnRails/public/files/WCAlogo_50x50.png'
          )
          .setColor('#00FF00')
          .setDescription('')
          .addField('Ville', comp.city, true)
          .addField(
            'Pays',
            `__**${getName(comp.country_iso2)}**__ ${countryCodeEmoji(
              comp.country_iso2
            )}`,
            true
          )
          .addField(
            'Adresse',
            `[${comp.venue_address}](https://duckduckgo.com/?ia=maps&iaxm=maps&q=${comp.latitude_degrees},${comp.longitude_degrees})`
          )
          .addField('Competiteurs max', comp.competitor_limit)
          .addField(
            'Date',
            prettifyTwoDates(comp.start_date, comp.end_date),
            true
          )
          .addField(
            'Inscriptions',
            prettifyTwoDates(comp.registration_open, comp.registration_close),
            true
          )
          .setTimestamp(comp.announced_at),
        reactions: map((id) => eventToEmoji[id], comp.event_ids),
      }
    : {
        embed: new Discord.MessageEmbed()
          .setTitle(`**${comp.name}**`)
          .setURL(comp.url)
          .setThumbnail(
            'https://raw.githubusercontent.com/thewca/worldcubeassociation.org/e974e9020e8c8a1e562c57695b96b312efb8eafa/WcaOnRails/public/files/WCAlogo_50x50.png'
          )
          .setColor('#FF0000')
          .setDescription('La compétition a été anulée.')
          .setTimestamp(comp.cancelled_at),
        reactions: [],
      }
);

export { getNewCompetitions, formatCompetition };
