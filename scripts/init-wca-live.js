import mongoose from 'mongoose';

import Wcalive from '../app/models/wca-live.js';
import { getRecentRecords } from '../app/helpers/wca-live-helpers.js';

mongoose.connect(process.env.MONGO_URL || 'mongodb://localhost:27017/wca', {
  useNewUrlParser: true,
  useFindAndModify: false,
  useCreateIndex: true,
  useUnifiedTopology: true,
});

const newWcalive = (doc) => new Wcalive(doc).save();

getRecentRecords().then((response) => {
  newWcalive(response).then(() => console.log('finished'));
});
