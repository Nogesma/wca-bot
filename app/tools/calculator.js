import { always, divide, gte, ifElse } from "ramda";

const centisecondsToTime = (time) => {
  const t = divide(time, 100);
  const min = Math.floor(t / 60);
  let s = (t - min * 60).toFixed(2);
  if (min > 0 && s.length === 4) {
    s = "0" + s;
  }

  return `${min ? min + ":" : ""}${s}`;
};

const decodeMbldAttemptResult = ifElse(
  gte(0),
  always({ solved: 0, attempted: 0, centiseconds: 0 }),
  (value) => {
    const missed = value % 100;
    const points = 99 - (Math.floor(value / 1e7) % 100);
    const solved = points + missed;
    return {
      solved,
      attempted: solved + missed,
      centiseconds: (Math.floor(value / 100) % 1e5) * 100,
    };
  }
);

export { centisecondsToTime, decodeMbldAttemptResult };
