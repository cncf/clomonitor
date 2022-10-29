import { groupBy } from 'lodash';
import moment from 'moment';

import { SortedDates } from '../types';

const groupDatesByYearAndMonth = (dates: string[]): SortedDates => {
  let sortedDates: SortedDates = {};

  const sortedByYears = groupBy(dates, (result: string) => moment(result, 'YYYY-MM-DD').endOf('year').format('YYYY'));
  const years = Object.keys(sortedByYears);
  years.forEach((year: string) => {
    const groupedByMonth = groupBy(sortedByYears[year], (result: string) =>
      moment(result, 'YYYY-MM-DD').endOf('month').format('MM')
    );
    sortedDates[year] = groupedByMonth;
  });

  return sortedDates;
};

export default groupDatesByYearAndMonth;
