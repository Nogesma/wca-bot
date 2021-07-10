import mongoose from 'mongoose';

const wcaliveSubSchema = new mongoose.Schema(
  {
    attemptResult: Number,
    result: {
      person: {
        country: {
          name: String,
        },
        name: String,
      },
      round: {
        competitionEvent: {
          competition: {
            id: String,
            name: String,
          },
          event: {
            id: String,
            name: String,
          },
        },
        id: String,
      },
    },
    tag: { type: String, enum: ['NR', 'CR', 'WR'] },
    type: { type: String, enum: ['single', 'average'] },
  },
  { _id: false }
);

const wcaliveSchema = new mongoose.Schema(
  {
    recentRecords: [wcaliveSubSchema],
  },
  { versionKey: false }
);

export default mongoose.model('Wcalive', wcaliveSchema);
