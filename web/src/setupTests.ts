import '@testing-library/jest-dom/vitest';

import { createRequire } from 'node:module';

import { vi } from 'vitest';

const require = createRequire(import.meta.url);

const noop = () => {};
Object.defineProperty(window, 'scrollTo', { value: noop, writable: true });

class ResizeObserverMock {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  constructor(private callback: any) {}
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  observe(_target: Element) {}
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  unobserve(_target: Element) {}
  disconnect() {}
}

Object.defineProperty(window, 'ResizeObserver', {
  value: ResizeObserverMock,
  writable: true,
});

const jestLike = Object.assign(vi, {
  requireActual: (id: string) => require(id),
});

Object.defineProperty(globalThis, 'jest', {
  value: jestLike,
  configurable: true,
});

vi.mock('clo-ui/components/CodeBlock', () => {
  return require('./__mocks__/clo-ui/components/CodeBlock');
});

vi.mock('react-apexcharts', () => ({
  __esModule: true,
  default: vi.fn(() => 'Chart'),
}));
