import { getRecentRecords } from '../helpers/wca-live-helpers.js';
import { getWcalive, updateWcalive } from './db-controller.js';
import fs from 'fs';
import { difference, equals } from 'ramda';

const formatJSON = (json) => JSON.parse(JSON.stringify(json));
const getNewRecords = async () => {
  const newRecords = await getRecentRecords();

  const recentRecords = formatJSON(newRecords.recentRecords);

  const oldRecords = formatJSON(await getWcalive());

  await updateWcalive(newRecords);
  console.log('up');

  const diff = difference(recentRecords, oldRecords);
  console.log(diff);
};

export { getNewRecords };
