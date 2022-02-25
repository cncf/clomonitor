import getCategoryColor from './getCategoryColor';

interface Test {
  input: number;
  output: string;
}

const tests: Test[] = [
  { input: 0, output: 'red' },
  { input: 24, output: 'red' },
  { input: 25, output: 'orange' },
  { input: 49, output: 'orange' },
  { input: 50, output: 'yellow' },
  { input: 74, output: 'yellow' },
  { input: 75, output: 'green' },
  { input: 100, output: 'green' },
  { input: 1000, output: 'green' },
  { input: -1, output: 'red' },
];

describe('getCategoryColor', () => {
  for (let i = 0; i < tests.length; i++) {
    it('returns color', () => {
      const actual = getCategoryColor(tests[i].input);
      expect(actual).toEqual(tests[i].output);
    });
  }
});
