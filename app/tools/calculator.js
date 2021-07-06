import { divide } from 'ramda';

const centisecondsToTime = (time) => {
  const t = divide(time, 100);
  const min = Math.floor(t / 60);
  let s = (t - min * 60).toFixed(2);
  if (min > 0 && s.length === 4) {
    s = '0' + s;
  }

  return `${min ? min + ':' : ''}${s}`;
};

export { centisecondsToTime };
