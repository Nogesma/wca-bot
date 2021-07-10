import Wcalive from '../models/wca-live.js';
import Wcacomp from '../models/wca-comp.js';
import { prop } from 'ramda';

const updateWcalive = (doc) => Wcalive.findOneAndReplace({}, doc).exec();

const getWcalive = async () =>
  prop('recentRecords', await Wcalive.findOne({}).exec());

const updateWcacomp = (doc) =>
  Wcacomp.findOneAndReplace({}, { competitions: doc }).exec();

const getWcacomp = async () => await Wcacomp.findOne({}).exec();

export { updateWcalive, getWcalive, updateWcacomp, getWcacomp };
