// This function is used to remove the inconsistencies between mongodb and
// graphql json, which allows for easier comparison
const formatJSON = (json) => JSON.parse(JSON.stringify(json));

const eventToEmoji = {
  333: '<:3x3solved:693841238461382739>',
  222: '<:2x2x2:693863867826176080>',
  444: '<:4x4x4:693863938915565599>',
  555: '<:5x5x5:861727566973239326>',
  666: '<:6x6x6:863917534108319744>',
  777: '<:7x7x7:861693153944535070>',
  skewb: '<:skewb:693893030998179911>',
  minx: '<:megaminx:693869410292727940>',
  sq1: '<:squane:693920069952077885>',
  '333oh': '🖐️',
  '333bf': '<:3BLD:706123835862417409>',
  '444bf': '<:4BLD:706123868556754976>',
  '555bf': '<:5BLD:706124832026132501>',
  '333fm': '✍️',
  '333mbf': '🧠',
  clock: '🕐',
  pyram: '<:pyraminx:693881128981102682>',
};

export { formatJSON, eventToEmoji };
