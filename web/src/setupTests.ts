import { createRequire } from 'module';
import { createElement, useEffect, useMemo } from 'react';
import { vi } from 'vitest';

import '@testing-library/jest-dom/vitest';

process.env.TZ = 'UTC';

vi.mock('/src/media/clomonitor.svg', () => ({
  __esModule: true,
  default: '/src/media/clomonitor.svg',
}));

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

const storage: Record<string, string> = {};
const localStorageMock = {
  getItem: (key: string) => (key in storage ? storage[key] : null),
  setItem: (key: string, value: string) => {
    storage[key] = value;
  },
  removeItem: (key: string) => {
    delete storage[key];
  },
  clear: () => {
    Object.keys(storage).forEach((key) => {
      delete storage[key];
    });
  },
};

Object.defineProperty(window, 'localStorage', {
  value: localStorageMock,
  writable: true,
});

Object.defineProperty(globalThis, 'localStorage', {
  value: localStorageMock,
  writable: true,
});

const jestLike = Object.assign(vi, {
  requireActual: (id: string) => require(id),
});

Object.defineProperty(globalThis, 'jest', {
  value: jestLike,
  configurable: true,
});

vi.mock('react-apexcharts', () => ({
  __esModule: true,
  default: vi.fn(() => 'Chart'),
}));

vi.mock('./layout/common/DateRangeFilter', () => {
  const formatDate = (source: Date) => {
    const formatter = new Intl.DateTimeFormat('en-US', {
      month: 'short',
      day: 'numeric',
      year: 'numeric',
      timeZone: 'UTC',
    });
    return formatter.format(source);
  };

  const DateRangeFilter = (props: {
    initialDate: string;
    from?: string;
    to?: string;
    onDateRangeChange: (value: { from?: string; to?: string }) => void;
  }) => {
    const fromDate = useMemo(
      () => (props.from ? new Date(props.from) : new Date(`${props.initialDate}T00:00:00.000Z`)),
      [props.from, props.initialDate]
    );
    const toDate = useMemo(() => {
      if (props.to) {
        return new Date(props.to);
      }
      const now = new Date();
      return new Date(Date.UTC(now.getUTCFullYear(), now.getUTCMonth(), now.getUTCDate()));
    }, [props.to]);

    useEffect(() => {
      props.onDateRangeChange({ from: props.from, to: props.to });
    }, [props.from, props.to, props.onDateRangeChange]);

    const content = [
      createElement(
        'div',
        { key: 'from-1' },
        createElement('label', { key: 'label' }, 'From:'),
        createElement('span', { key: 'value' }, formatDate(fromDate))
      ),
      createElement(
        'div',
        { key: 'to-1' },
        createElement('label', { key: 'label' }, 'To:'),
        createElement('span', { key: 'value' }, formatDate(toDate))
      ),
      createElement(
        'div',
        { key: 'from-2' },
        createElement('label', { key: 'label' }, 'From:'),
        createElement('span', { key: 'value' }, formatDate(fromDate))
      ),
      createElement(
        'div',
        { key: 'to-2' },
        createElement('label', { key: 'label' }, 'To:'),
        createElement('span', { key: 'value' }, formatDate(toDate))
      ),
    ];

    return createElement('div', null, content);
  };

  return {
    __esModule: true,
    default: DateRangeFilter,
  };
});

vi.mock('./layout/common/DateRangeBtn', () => {
  const DateRange = {
    From: 'from',
    To: 'to',
  };

  const DateRangeBtn = () => createElement('button', { type: 'button' }, 'Date range');

  return {
    __esModule: true,
    DateRange,
    default: DateRangeBtn,
  };
});

const fetchMock = vi.fn(
  async () =>
    new Response(null, {
      status: 200,
      statusText: 'OK',
    })
);

vi.stubGlobal('fetch', fetchMock);
