import { graphql } from 'graphql';
import { UrlLoader, loadSchema } from 'graphql-tools';
import { getCode } from 'country-list';
import countryCodeEmoji from 'country-code-emoji';

import dayjs from 'dayjs';
import { flatten, map, omit, range, equals, includes } from 'ramda';
import fetch from 'node-fetch';

const getSchema = await loadSchema(
  'https://live.worldcubeassociation.org/api/graphql',
  {
    loaders: [new UrlLoader()],
  }
);

const getRecentRecords = async () =>
  graphql(
    await getSchema,
    `
      {
        recentRecords {
          type
          tag
          attemptResult
          result {
            person {
              name
              country {
                name
              }
            }
            round {
              id
              competitionEvent {
                event {
                  id
                  name
                }
                competition {
                  id
                  name
                }
              }
            }
          }
        }
      }
    `
  ).then((response) => response.data);

const getUpcomingCompetitions = async () => {
  const url = `https://www.worldcubeassociation.org/api/v0/competitions?sort=start_date&start=${dayjs().format(
    'YYYY-MM-DD'
  )}`;

  const { response: firstResponse, numberOfPages } = await fetch(url).then(
    async (res) => {
      const numberOfPages = Math.ceil(
        res.headers.get('Total') / res.headers.get('Per-page')
      );
      const response = await res.json();
      return { response, numberOfPages };
    }
  );

  const responseArray = await Promise.all(
    map(
      (n) => fetch(url + '&page=' + n).then((res) => res.json()),
      range(2, numberOfPages + 1)
    )
  );

  return map(
    omit(['delegates', 'trainee_delegates', 'organizers']),
    flatten([firstResponse, responseArray])
  );
};

const prettifyTwoDates = (date1, date2) => {
  const formattedDate = dayjs(date2).format('DD/MM/YYYY');

  return dayjs(date1).format('DD/MM/YYYY') === formattedDate
    ? formattedDate
    : `${dayjs(date1).format('DD')} au ${formattedDate}`;
};
// This function is used to remove the inconsistencies between mongodb and
// graphql json, which allows for easier comparison
const formatJSON = (json) => JSON.parse(JSON.stringify(json));

const countryNameToFlagEmoji = (name) => countryCodeEmoji(getCode(name));

const eventToEmoji = {
  333: '<:3x3solved:693841238461382739>',
  222: '<:2x2x2:693863867826176080>',
  444: '<:4x4x4:693863938915565599>',
  555: '<:5x5x5:861727566973239326>',
  666: '<:6x6x6:861708396486983730>',
  777: '<:7x7x7:861693153944535070>',
  skewb: '<:skewb:693893030998179911>',
  minx: '<:megaminx:693869410292727940>',
  sq1: '<:squane:693920069952077885>',
  '333oh': '🖐️',
  '333bf': '<:3BLD:706123835862417409>',
  '444bf': '<:4BLD:706123868556754976>',
  '555bf': '<:5BLD:706124832026132501>',
  '333fm': '✍️',
  '333mbf': '🧠',
  clock: '🕐',
  pyram: '<:pyraminx:693881128981102682>',
};

const getColorOfTag = {
  WR: '#f44336',
  CR: '#ffeb3b',
  NR: '#00e676',
};

const getResultType = (t, e) =>
  equals('average', t)
    ? includes(e, ['666', '777', '333bf', '444bf', '555bf', '333fm'])
      ? 'mean'
      : t
    : t;

export {
  getRecentRecords,
  getUpcomingCompetitions,
  formatJSON,
  countryNameToFlagEmoji,
  eventToEmoji,
  getColorOfTag,
  getResultType,
  prettifyTwoDates,
};
