import { cond, pipe, prop, propEq, split, when } from "ramda";

import { sendNewCompMessage } from "../helpers/message-handler.js";

const messageIsCommand = (mess) =>
  mess.member.roles.cache.some(
    (role) =>
      role.permissions.has("ADMINISTRATOR") || role.id === process.env.MOD_ID,
  );

const commandChoose = cond([
  [propEq("command")("!newcomp"), sendNewCompMessage],
]);

const applyCommand = (message) => {
  const { author, channel } = message;
  const [command, ...args] = pipe(prop("content"), split(" "))(message);

  return commandChoose({
    author,
    channel,
    command,
    args,
  });
};

const incomingMessage = when(messageIsCommand)(applyCommand);

export { incomingMessage };
