import { render } from '@testing-library/react';

import getMetaTag from './getMetaTag';

interface Test {
  name: string;
  value: string | boolean;
  isTrue?: boolean;
}

const tests: Test[] = [
  {
    name: 'primaryColor',
    value: '#f0a',
  },
  {
    name: 'secondaryColor',
    value: '#2a0552',
  },
];

describe('getMetaTag', () => {
  for (let i = 0; i < tests.length; i++) {
    it('returns proper value', () => {
      render(<meta name={`clomonitor:${tests[i].name}`} content={`${tests[i].value.toString()}`} />);
      const actual = getMetaTag(tests[i].name, tests[i].isTrue);
      expect(actual).toEqual(tests[i].value);
    });
  }
});
