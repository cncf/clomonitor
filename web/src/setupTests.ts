import '@testing-library/jest-dom/vitest';

import { createRequire } from 'module';
import { createElement, type ReactNode } from 'react';
import { vi } from 'vitest';

const require = createRequire(import.meta.url);

const noop = () => {};
Object.defineProperty(window, 'scrollTo', { value: noop, writable: true });

class ResizeObserverMock {
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

vi.mock('clo-ui/components/CodeBlock', () => ({
  __esModule: true,
  CodeBlock: ({
    content,
    label,
    withCopyBtn,
  }: {
    content: ReactNode;
    label?: string;
    withCopyBtn?: boolean;
  }) =>
    createElement(
      'div',
      { 'data-testid': 'code', 'data-label': label },
      content,
      withCopyBtn && label
        ? createElement('button', { 'aria-label': label, type: 'button' }, label)
        : null
    ),
}));

vi.mock('react-apexcharts', () => ({
  __esModule: true,
  default: vi.fn(() => 'Chart'),
}));

const fetchMock = vi.fn(
  async () =>
    new Response(null, {
      status: 200,
      statusText: 'OK',
    })
);

vi.stubGlobal('fetch', fetchMock);
