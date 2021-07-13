import dayjs from 'dayjs';
import fetch from 'node-fetch';
import { flatten, map, omit, range } from 'ramda';

const getUpcomingCompetitions = async () => {
  const url = `https://www.worldcubeassociation.org/api/v0/competitions?sort=start_date&start=${dayjs().format(
    'YYYY-MM-DD'
  )}`;

  const { response: firstResponse, numberOfPages } = await fetch(url).then(
    async (res) => {
      const numberOfPages = Math.ceil(
        res.headers.get('Total') / res.headers.get('Per-page')
      );
      const response = await res.json();
      return { response, numberOfPages };
    }
  );

  const responseArray = await Promise.all(
    map(
      (n) => fetch(url + '&page=' + n).then((res) => res.json()),
      range(2, numberOfPages + 1)
    )
  );

  return map(
    omit(['delegates', 'trainee_delegates', 'organizers']),
    flatten([firstResponse, responseArray])
  );
};

const prettifyTwoDates = (date1, date2) => {
  const formattedDate = dayjs(date2).format('DD/MM/YYYY');

  return dayjs(date1).format('DD/MM/YYYY') === formattedDate
    ? formattedDate
    : `${dayjs(date1).format('DD')} au ${formattedDate}`;
};

export { getUpcomingCompetitions, prettifyTwoDates };
