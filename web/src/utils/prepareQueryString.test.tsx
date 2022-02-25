import { SearchFiltersURL } from '../types';
import prepareQueryString from './prepareQueryString';

interface Test {
  input: SearchFiltersURL;
  output: string;
}

const tests: Test[] = [
  { input: { pageNumber: 1 }, output: '?page=1' },
  { input: { pageNumber: 1, text: 'test' }, output: '?text=test&page=1' },
  { input: { pageNumber: 1, text: 'test', filters: { category: ['0'] } }, output: '?category=0&text=test&page=1' },
  {
    input: { pageNumber: 2, text: 'test', filters: { category: ['0', '2'] } },
    output: '?category=0&category=2&text=test&page=2',
  },
  {
    input: { pageNumber: 1, text: 'test', filters: { category: ['0', '2'], maturity: ['0'], rating: ['a', 'b'] } },
    output: '?category=0&category=2&maturity=0&rating=a&rating=b&text=test&page=1',
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
