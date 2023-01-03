import getAnchorValue from './getAnchorValue';

interface Test {
  input: string;
  result?: string;
}

const tests: Test[] = [
  { input: '', result: '' },
  { input: 'Title', result: 'title' },
  { input: 'Long title', result: 'long-title' },
  { input: 'Configure TEST', result: 'configure-test' },
  { input: 'TL;DR;', result: 'tl-dr' },
  { input: 'FAQs', result: 'faqs' },
  {
    input: '2. Title.',
    result: 'X-title',
  },
  {
    input: '可选：微信推送打卡结果',
    result: '可选-微信推送打卡结果',
  },
  {
    input: '[2.3.1]',
    result: 'X-3-1',
  },
];

describe('getAnchorValue', () => {
  for (let i = 0; i < tests.length; i++) {
    it('returns proper string', () => {
      const actual = getAnchorValue(tests[i].input);
      expect(actual).toEqual(tests[i].result);
    });
  }
});
