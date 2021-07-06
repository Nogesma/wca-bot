import Wcalive from '../models/wca-live.js';
import { prop } from 'ramda';

const updateWcalive = (doc) => Wcalive.findOneAndReplace({}, doc).exec();

const getWcalive = async () =>
  prop('recentRecords', await Wcalive.findOne({}).exec());

export { updateWcalive, getWcalive };
