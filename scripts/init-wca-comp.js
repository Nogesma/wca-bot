import mongoose from 'mongoose';

import Wcacomp from '../app/models/wca-comp.js';
import { getUpcomingCompetitions } from '../app/helpers/wca-comp-helpers.js';

mongoose.connect(process.env.MONGO_URL || 'mongodb://localhost:27017/wca', {
  useNewUrlParser: true,
  useFindAndModify: false,
  useCreateIndex: true,
  useUnifiedTopology: true,
});

const newWcacomp = (doc) => new Wcacomp({ competitions: doc }).save();

getUpcomingCompetitions().then((response) => {
  newWcacomp(response).then(() => console.log('finished'));
});
