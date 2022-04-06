import { SearchFiltersURL } from '../types';
import prepareQueryString from './prepareQueryString';

interface Test {
  input: SearchFiltersURL;
  output: string;
}

const tests: Test[] = [
  { input: { pageNumber: 1 }, output: '?page=1' },
  { input: { pageNumber: 1, text: 'test' }, output: '?text=test&page=1' },
  {
    input: { pageNumber: 1, text: 'test', filters: { maturity: ['sandbox'] } },
    output: '?maturity=sandbox&text=test&page=1',
  },
  {
    input: { pageNumber: 2, text: 'test', filters: { foundation: ['lfaidata'] } },
    output: '?foundation=lfaidata&text=test&page=2',
  },
  {
    input: {
      pageNumber: 1,
      text: 'test',
      filters: { foundation: ['cncf'], maturity: ['graduated'], rating: ['a', 'b'] },
    },
    output: '?foundation=cncf&maturity=graduated&rating=a&rating=b&text=test&page=1',
  },
  {
    input: {
      pageNumber: 2,
      text: 'test',
      accepted_from: '2020-01-01',
      accepted_to: '2020-12-31',
      filters: { maturity: ['graduated'] },
    },
    output: '?maturity=graduated&text=test&accepted_from=2020-01-01&accepted_to=2020-12-31&page=2',
  },
];

describe('prepareQueryString', () => {
  for (let i = 0; i < tests.length; i++) {
    it('returns data', () => {
      const actual = prepareQueryString(tests[i].input);
      expect(actual).toEqual(tests[i].output);
    });
  }
});
