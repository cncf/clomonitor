import { createRequire } from 'node:module';

import { render, screen, waitFor } from '@testing-library/react';
import { BrowserRouter as Router } from 'react-router-dom';
import { vi } from 'vitest';

import API from '../../api';
import { Project } from '../../types';

const filtersRenderSpy = vi.fn();

vi.mock('./filters', () => ({
  __esModule: true,
  default: (props: {
    onChange: (name: string, value: string, checked: boolean) => void;
    onAcceptedDateRangeChange: (range: { from?: string; to?: string }) => void;
  }) => {
    filtersRenderSpy(props);
    return (
      <div data-testid="filters-mock">
        <button type="button" onClick={() => props.onChange('rating', 'a', true)}>
          add-rating-filter
        </button>
        <button type="button" onClick={() => props.onAcceptedDateRangeChange({ from: '2020-01-01' })}>
          change-date-range
        </button>
      </div>
    );
  },
}));

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

describe('Project detail index', () => {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  let dateNowSpy: any;
  const searchProjectsMock = vi.spyOn(API, 'searchProjects');

  beforeEach(() => {
    dateNowSpy = vi.spyOn(Date, 'now').mockImplementation(() => 1634968825000);
  });

  afterAll(() => {
    dateNowSpy.mockRestore();
  });

  afterEach(() => {
    searchProjectsMock.mockReset();
    filtersRenderSpy.mockReset();
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

    await screen.findAllByRole('listitem');
    expect(API.searchProjects).toHaveBeenCalledTimes(1);
    expect(asFragment()).toMatchSnapshot();
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
