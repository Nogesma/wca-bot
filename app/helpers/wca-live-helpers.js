import { countryToAlpha2 } from 'country-to-iso';
import countryCodeEmoji from 'country-code-emoji';
import {
  cond,
  equals,
  ifElse,
  includes,
  lt,
  pipe,
  prop,
  propEq,
  T,
  toString,
} from 'ramda';
import { API, graphqlOperation } from 'aws-amplify';
import { gql } from 'graphql-tag';
import {
  centisecondsToTime,
  decodeMbldAttemptResult,
} from '../tools/calculator.js';

const query = gql`
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
`;

const getRecentRecords = () =>
  API.graphql(graphqlOperation(query)).then((res) => res.data.recentRecords);

const countryNameToFlagEmoji = pipe(countryToAlpha2, countryCodeEmoji);

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

/* Note: FM singles are stored as the number of moves (e.g. 25),
   while averages are stored with 2 decimal places (e.g. 2533 for an average of 25.33 moves). */
const formatFmAttemptResult = pipe(
  prop('attemptResult'),
  ifElse(lt(1000), centisecondsToTime, toString)
);

const formatMbldAttemptResult = pipe(
  prop('attemptResult'),
  decodeMbldAttemptResult,
  ({ solved, attempted, centiseconds }) =>
    `${solved}/${attempted} ${centisecondsToTime(centiseconds).replace(
      /\.00$/,
      ''
    )}`
);

const formatAttemptResult = cond([
  [propEq('eventId')('333fm'), formatFmAttemptResult],
  [propEq('eventId')('333mbf'), formatMbldAttemptResult],
  [T, pipe(prop('attemptResult'), centisecondsToTime)],
]);

export {
  getRecentRecords,
  countryNameToFlagEmoji,
  getColorOfTag,
  getResultType,
  formatAttemptResult,
};
