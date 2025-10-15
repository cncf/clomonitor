// react-testing-library renders your components to document.body,
// this adds jest-dom's custom assertions
import '@testing-library/jest-dom';

const noop = () => {};
Object.defineProperty(window, 'scrollTo', { value: noop, writable: true });

jest.mock('clo-ui/components/CodeBlock', () => {
  return jest.requireActual('./__mocks__/clo-ui/components/CodeBlock');
});
