import mongoose from 'mongoose';

const wcacompSubSchema = new mongoose.Schema(
  {
    class: String,
    url: String,
    id: String,
    name: String,
    website: String,
    short_name: String,
    city: String,
    venue_address: String,
    venue_details: String,
    latitude_degrees: Number,
    longitude_degrees: Number,
    country_iso2: String,
    start_date: Date,
    registration_open: Date,
    registration_close: Date,
    announced_at: Date,
    cancelled_at: Date,
    end_date: Date,
    competitor_limit: Number,
    event_ids: [String],
  },
  { _id: false }
);

const wcacompSchema = new mongoose.Schema(
  {
    competitions: [wcacompSubSchema],
  },
  { versionKey: false }
);

export default mongoose.model('Wcacomp', wcacompSchema);
