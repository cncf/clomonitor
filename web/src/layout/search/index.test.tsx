import { render, screen, waitFor } from '@testing-library/react';
import { createRequire } from 'module';
import { BrowserRouter as Router } from 'react-router-dom';
import { vi } from 'vitest';

import API from '../../api';
import { Project } from '../../types';
import Search from './index';

const require = createRequire(import.meta.url);

const getMockSearch = (fixtureId: string): { items: Project[]; 'Pagination-Total-Count': string } => {
  return require(`./__fixtures__/index/${fixtureId}.json`) as { items: Project[]; 'Pagination-Total-Count': string };
};

const defaultProps = {
  scrollPosition: 0,
  setScrollPosition: vi.fn(),
  setInvisibleFooter: vi.fn(),
};

const searchProjectsMock = vi.spyOn(API, 'searchProjects');

const realDate = Date;

const freezeDate = (isoDate: string) => {
  const fixedDate = new realDate(isoDate);

  const MockDate = class extends realDate {
    constructor(value?: number | string | Date) {
      if (arguments.length === 0) {
        return new realDate(fixedDate);
      }
      return new realDate(value as number | string | Date);
    }

    static now(): number {
      return fixedDate.getTime();
    }

    static parse(dateString: string): number {
      return realDate.parse(dateString);
    }

    static UTC(...args: Parameters<typeof realDate.UTC>): number {
      return realDate.UTC(...args);
    }
  };

  Object.setPrototypeOf(MockDate, realDate);
  // @ts-expect-error overriding global Date for tests
  global.Date = MockDate as unknown as DateConstructor;
};

describe('Project detail index', () => {
  beforeEach(() => {
    freezeDate('2021-10-23T00:00:00.000Z');
  });

  afterEach(() => {
    // @ts-expect-error restoring original Date
    global.Date = realDate;
    searchProjectsMock.mockReset();
    vi.resetAllMocks();
  });

  it('creates snapshot', async () => {
    const mockSearch = getMockSearch('1');
    searchProjectsMock.mockResolvedValue(mockSearch);

    const { asFragment } = render(
      <Router>
        <Search {...defaultProps} />
      </Router>
    );

    await waitFor(() => {
      expect(API.searchProjects).toHaveBeenCalledTimes(1);
      expect(asFragment()).toMatchSnapshot();
    });
  });

  describe('Render', () => {
    it('renders component', async () => {
      const mockSearch = getMockSearch('1');
      searchProjectsMock.mockResolvedValue(mockSearch);

      render(
        <Router>
          <Search {...defaultProps} />
        </Router>
      );

      await waitFor(() => {
        expect(API.searchProjects).toHaveBeenCalledTimes(1);
        expect(API.searchProjects).toHaveBeenCalledWith({
          filters: {},
          limit: 20,
          offset: 0,
          sort_by: 'name',
          sort_direction: 'asc',
          text: undefined,
        });
      });

      expect(screen.getAllByRole('listitem')).toHaveLength(4);
    });

    it('renders placeholder when list is empty', async () => {
      const mockSearch = getMockSearch('2');
      searchProjectsMock.mockResolvedValue(mockSearch);

      render(
        <Router>
          <Search {...defaultProps} />
        </Router>
      );

      await waitFor(() => {
        expect(API.searchProjects).toHaveBeenCalledTimes(1);
      });

      expect(screen.getByText(/We're sorry!/)).toBeInTheDocument();
      expect(screen.getByText(/We can't seem to find any projects that match your search/)).toBeInTheDocument();
    });
  });
});
