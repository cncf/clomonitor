import { SearchFiltersURL } from '../types';
import buildSearchParams from './buildSearchParams';

interface Test {
  input: URLSearchParams;
  output: SearchFiltersURL;
}

const tests: Test[] = [
  { input: new URLSearchParams(''), output: { text: undefined, filters: {}, pageNumber: 1 } },
  { input: new URLSearchParams('?page=3'), output: { text: undefined, filters: {}, pageNumber: 3 } },
  {
    input: new URLSearchParams('?page=1&maturity=1'),
    output: { text: undefined, filters: { maturity: ['1'] }, pageNumber: 1 },
  },
  {
    input: new URLSearchParams('?page=1&maturity=0&maturity=1'),
    output: { text: undefined, filters: { maturity: ['0', '1'] }, pageNumber: 1 },
  },
  {
    input: new URLSearchParams('?page=1&maturity=0&category=1&rating=b'),
    output: { text: undefined, filters: { maturity: ['0'], category: ['1'], rating: ['b'] }, pageNumber: 1 },
  },
  {
    input: new URLSearchParams('?page=1&maturity=0&category=1&rating=b&text=test'),
    output: { text: 'test', filters: { maturity: ['0'], category: ['1'], rating: ['b'] }, pageNumber: 1 },
  },
  {
    input: new URLSearchParams('?page=1&maturity=0&maturity=1&accepted_from=2020-01-01'),
    output: { text: undefined, accepted_from: '2020-01-01', filters: { maturity: ['0', '1'] }, pageNumber: 1 },
  },
  {
    input: new URLSearchParams('?page=1&maturity=0&maturity=1&accepted_to=2019-12-31'),
    output: { text: undefined, accepted_to: '2019-12-31', filters: { maturity: ['0', '1'] }, pageNumber: 1 },
  },
];

describe('buildSearchParams', () => {
  for (let i = 0; i < tests.length; i++) {
    it('returns data', () => {
      const actual = buildSearchParams(tests[i].input);
      expect(actual).toEqual(tests[i].output);
    });
  }
});
