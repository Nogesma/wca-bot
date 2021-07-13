import { graphql } from 'graphql';
import { UrlLoader, loadSchema } from 'graphql-tools';
import { getCode } from 'country-list';
import countryCodeEmoji from 'country-code-emoji';
import { equals, includes } from 'ramda';

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

const countryNameToFlagEmoji = (name) => countryCodeEmoji(getCode(name));

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
  countryNameToFlagEmoji,
  getColorOfTag,
  getResultType,
};
