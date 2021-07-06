import { graphql } from 'graphql';
import { UrlLoader, loadSchema } from 'graphql-tools';

const getSchema = await loadSchema(
  'https://live.worldcubeassociation.org/api/graphql',
  {
    // load from endpoint
    loaders: [new UrlLoader()],
  }
);

const getRecentRecords = async () =>
  graphql(
    await getSchema,
    '{ recentRecords { type tag attemptResult result { person { name country { name } } round { id competitionEvent { event { id name } competition { id name } } } } } }'
  ).then((response) => response.data);

export { getRecentRecords };
