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
    input: new URLSearchParams('?page=1&maturity=incubating'),
    output: { text: undefined, filters: { maturity: ['incubating'] }, pageNumber: 1 },
  },
  {
    input: new URLSearchParams('?page=1&maturity=graduated&maturity=incubating'),
    output: { text: undefined, filters: { maturity: ['graduated', 'incubating'] }, pageNumber: 1 },
  },
  {
    input: new URLSearchParams('?page=1&maturity=graduated&foundation=cncf&rating=b'),
    output: {
      text: undefined,
      filters: { maturity: ['graduated'], foundation: ['cncf'], rating: ['b'] },
      pageNumber: 1,
    },
  },
  {
    input: new URLSearchParams('?page=1&maturity=graduated&foundation=cncf&rating=b&text=test'),
    output: { text: 'test', filters: { maturity: ['graduated'], foundation: ['cncf'], rating: ['b'] }, pageNumber: 1 },
  },
  {
    input: new URLSearchParams('?page=1&maturity=graduated&maturity=incubating&accepted_from=2020-01-01'),
    output: {
      text: undefined,
      accepted_from: '2020-01-01',
      filters: { maturity: ['graduated', 'incubating'] },
      pageNumber: 1,
    },
  },
  {
    input: new URLSearchParams('?page=1&maturity=graduated&maturity=incubating&accepted_to=2019-12-31'),
    output: {
      text: undefined,
      accepted_to: '2019-12-31',
      filters: { maturity: ['graduated', 'incubating'] },
      pageNumber: 1,
    },
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
