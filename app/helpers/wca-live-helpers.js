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
} from "ramda";
import { gql, GraphQLClient } from "graphql-request";

import {
  centisecondsToTime,
  decodeMbldAttemptResult,
} from "../tools/calculator.js";

const client = new GraphQLClient(
  "https://live.worldcubeassociation.org/api/graphql",
);

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
            iso2
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
  client.request(query).then((res) => res.recentRecords);

const getColorOfTag = {
  WR: "#f44336",
  CR: "#ffeb3b",
  NR: "#00e676",
};

const getResultType = (t, e) =>
  equals("average", t)
    ? includes(e, ["666", "777", "333bf", "444bf", "555bf", "333fm"])
      ? "mean"
      : t
    : t;

/* Note: FM singles are stored as the number of moves (e.g. 25),
   while averages are stored with 2 decimal places (e.g. 2533 for an average of 25.33 moves). */
const formatFmAttemptResult = pipe(
  prop("attemptResult"),
  ifElse(lt(1000), centisecondsToTime, toString),
);

const formatMbldAttemptResult = pipe(
  prop("attemptResult"),
  decodeMbldAttemptResult,
  ({ solved, attempted, centiseconds }) =>
    `${solved}/${attempted} ${centisecondsToTime(centiseconds).replace(
      /\.00$/,
      "",
    )}`,
);

const formatAttemptResult = cond([
  [propEq("eventId")("333fm"), formatFmAttemptResult],
  [propEq("eventId")("333mbf"), formatMbldAttemptResult],
  [T, pipe(prop("attemptResult"), centisecondsToTime)],
]);

export { getRecentRecords, getColorOfTag, getResultType, formatAttemptResult };
