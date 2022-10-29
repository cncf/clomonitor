import { SortedDates } from '../types';
import groupDatesByYearAndMonth from './groupDatesByYearAndMonth';

interface Test {
  input: string[];
  output: SortedDates;
}

const tests: Test[] = [
  { input: [], output: {} },
  {
    input: ['2022-10-28', '2022-10-27', '2022-10-26', '2022-09-10', '2022-08-07', '2021-04-18', '2020-03-09'],
    output: {
      '2022': {
        '08': ['2022-08-07'],
        '09': ['2022-09-10'],
        '10': ['2022-10-28', '2022-10-27', '2022-10-26'],
      },
      '2021': {
        '04': ['2021-04-18'],
      },
      '2020': {
        '03': ['2020-03-09'],
      },
    },
  },
  {
    input: ['2020-10-28', '2022-10-27', '2018-10-26', '2022-09-10', '2022-08-07', '2021-04-18', '2019-03-09'],
    output: {
      '2022': {
        '08': ['2022-08-07'],
        '09': ['2022-09-10'],
        '10': ['2022-10-27'],
      },
      '2020': {
        '10': ['2020-10-28'],
      },
      '2021': {
        '04': ['2021-04-18'],
      },
      '2019': {
        '03': ['2019-03-09'],
      },
      '2018': {
        '10': ['2018-10-26'],
      },
    },
  },
];

describe('groupDatesByYearAndMonth', () => {
  for (let i = 0; i < tests.length; i++) {
    it('returns grouped dates', () => {
      const actual = groupDatesByYearAndMonth(tests[i].input);
      expect(actual).toEqual(tests[i].output);
    });
  }
});
