import { getWcacomp } from '../controllers/db-controller.js';
import { equals, filter, map, pipe, prop } from 'ramda';
import { formatCompetition } from '../controllers/wca-comp-controller.js';

const sendNewCompMessage = async ({ channel, args }) => {
  const compID = args[0];

  const competitions = await getWcacomp();

  const comp = filter(pipe(prop('id'), equals(compID)))(
    competitions.competitions
  );

  const [{ embed, reactions }] = formatCompetition(comp);

  const chan = await channel.guild.channels.cache.get(process.env.WCA_LIVE);

  chan.send(embed).then((mess) => map((emoji) => mess.react(emoji), reactions));
};

export { sendNewCompMessage };
