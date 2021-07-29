import {
  __,
  difference,
  filter,
  includes,
  map,
  prepend,
  propSatisfies,
} from 'ramda';
import countryCodeEmoji from 'country-code-emoji';
import * as Discord from 'discord.js';
import { getName } from 'country-list';

import { getWcacomp, updateWcacomp } from './db-controller.js';
import { eventToEmoji, formatJSON } from '../helpers/global-helpers.js';
import {
  getUpcomingCompetitions,
  prettifyTwoDates,
} from '../helpers/wca-comp-helpers.js';

const getNewCompetitions = async () => {
  const upcomingCompetitions = formatJSON(await getUpcomingCompetitions());

  const oldCompetitions = formatJSON(await getWcacomp()).competitions;

  await updateWcacomp(upcomingCompetitions);

  const newComps = difference(upcomingCompetitions, oldCompetitions);

  return filter(
    propSatisfies(
      includes(__, [
        'FR',
        'GB',
        'BE',
        'CH',
        'DE',
        'NL',
        'ES',
        'PT',
        'IT',
        'CA',
        'AD',
        'MC',
        'SM',
        'LU',
        'LI',
      ]),
      'country_iso2'
    ),
    newComps
  );
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
          ),
        reactions: prepend(
          '<:WCA:456059019677663233>',
          map((id) => eventToEmoji[id], comp.event_ids)
        ),
      }
    : {
        embed: new Discord.MessageEmbed()
          .setTitle(`**${comp.name}**`)
          .setURL(comp.url)
          .setThumbnail(
            'https://raw.githubusercontent.com/thewca/worldcubeassociation.org/e974e9020e8c8a1e562c57695b96b312efb8eafa/WcaOnRails/public/files/WCAlogo_50x50.png'
          )
          .setColor('#FF0000')
          .setDescription('La compétition a été annulée.'),
        reactions: ['<:RIP:421349840467787776>'],
      }
);

export { getNewCompetitions, formatCompetition };
