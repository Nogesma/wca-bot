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

const prettifyTwoDates = (startDateStr, endDateStr) => {
  const startDate = dayjs(startDateStr);
  const endDate = dayjs(endDateStr);
  const formattedDate = endDate.format('DD/MM/YYYY');

  if (startDate.year() === endDate.year()) {
    if (startDate.month() === endDate.month()) {
      if (startDate.day() === endDate.day()) {
        return formattedDate;
      }
      return `${startDate.format('DD')} au ${formattedDate}`;
    }
    return `${startDate.format('DD/MM')} au ${formattedDate}`;
  }
  return `${startDate.format('DD/MM/YYYY')} au ${formattedDate}`;
};

export { getUpcomingCompetitions, prettifyTwoDates };
