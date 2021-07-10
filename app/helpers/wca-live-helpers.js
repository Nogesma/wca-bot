import { graphql } from 'graphql';
import { UrlLoader, loadSchema } from 'graphql-tools';
import { getCode } from 'country-list';
import countryCodeEmoji from 'country-code-emoji';

const getSchema = await loadSchema(
  'https://live.worldcubeassociation.org/api/graphql',
  {
    loaders: [new UrlLoader()],
  }
);

const getRecentRecords = async () =>
  graphql(
    await getSchema,
    '{ recentRecords { type tag attemptResult result { person { name country { name } } round { id competitionEvent { event { id name } competition { id name } } } } } }'
  ).then((response) => response.data);

// This function is used to remove the inconsistencies between mongodb and
// graphql json, which allows for easier comparison
const formatJSON = (json) => JSON.parse(JSON.stringify(json));

const countryNameToFlagEmoji = (name) => countryCodeEmoji(getCode(name));

const tagToEmoji = {
  WR: '<:WR:369397960926298112>',
  CR: '<:CR:369398840928894976>',
  NR: '<:NR:369399233867939840>',
};

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
  oh: '🖐️',
  '3bld': '<:3BLD:706123835862417409>',
  '4bld': '<:4BLD:706123868556754976>',
  '5bld': '<:5BLD:706124832026132501>',
  fmc: '✍️',
  mbld: '🧠',
  pyram: '<:pyraminx:693881128981102682>',
};

export {
  getRecentRecords,
  formatJSON,
  countryNameToFlagEmoji,
  tagToEmoji,
  eventToEmoji,
};
