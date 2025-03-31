import { equals, filter, isEmpty, map, pipe, prop } from "ramda";

import { getWcacomp } from "../controllers/db-controller.js";
import { formatCompetition } from "../controllers/wca-comp-controller.js";
import { getAllThreads } from "./wca-comp-helpers.js";

const sendNewCompMessage = async ({ channel, args }) => {
  const compID = args[0];

  const competitions = await getWcacomp();

  const comp = filter(pipe(prop("id"), equals(compID)))(competitions);

  const formattedCompetition = formatCompetition(comp);

  if (!isEmpty(formattedCompetition)) {
    const [{ embed, reactions, country }] = formattedCompetition;

    const forum = await channel.guild.channels.fetch(process.env.WCA_COMP);
    const threads = await getAllThreads(forum);

    const chan = threads.find((x) => x.name === country);
    chan
      .send({ embeds: [embed] })
      .then((mess) => map((emoji) => mess.react(emoji), reactions));

    channel.send("La compétition a été annoncée.");
  } else {
    channel.send("Erreur, la compétition n'existe pas dans la db.");
  }
};

export { sendNewCompMessage };
