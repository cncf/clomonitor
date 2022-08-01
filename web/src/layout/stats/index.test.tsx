import { render, screen, waitFor } from '@testing-library/react';
import { mocked } from 'jest-mock';
import { BrowserRouter as Router } from 'react-router-dom';

import API from '../../api';
import { AppContext } from '../../context/AppContextProvider';
import { SortBy, SortDirection, Stats } from '../../types';
import StatsView from './index';
jest.mock('../../api');
jest.mock('react-apexcharts', () => () => <div>Chart</div>);

const getMockStats = (fixtureId: string): Stats => {
  return require(`./__fixtures__/index/${fixtureId}.json`) as Stats;
};

const mockCtx = {
  prefs: {
    search: { limit: 20, sort: { by: SortBy.Name, direction: SortDirection.ASC } },
    theme: { effective: 'light', configured: 'light' },
  },
};

describe('StatsView', () => {
  afterEach(() => {
    jest.resetAllMocks();
  });

  it('creates snapshot', async () => {
    const mockStats = getMockStats('1');
    mocked(API).getStats.mockResolvedValue(mockStats);

    const { asFragment } = render(
      <AppContext.Provider value={{ ctx: mockCtx, dispatch: jest.fn() }}>
        <Router>
          <StatsView />
        </Router>
      </AppContext.Provider>
    );

    await waitFor(() => {
      expect(API.getStats).toHaveBeenCalledTimes(1);
    });

    await waitFor(() => {
      expect(screen.getAllByText('Chart')).toHaveLength(6);
    });

    expect(asFragment()).toMatchSnapshot();
  });

  describe('Render', () => {
    it('renders component', async () => {
      const mockStats = getMockStats('1');
      mocked(API).getStats.mockResolvedValue(mockStats);

      render(
        <AppContext.Provider value={{ ctx: mockCtx, dispatch: jest.fn() }}>
          <Router>
            <StatsView />
          </Router>
        </AppContext.Provider>
      );

      await waitFor(() => {
        expect(API.getStats).toHaveBeenCalledTimes(1);
      });

      await waitFor(() => {
        expect(screen.getAllByText('Chart')).toHaveLength(6);
      });

      expect(screen.getByText('Report generated at:')).toBeInTheDocument();
      expect(screen.getByText('Projects')).toBeInTheDocument();
      expect(screen.getByText('Projects accepted')).toBeInTheDocument();
      expect(screen.getByText('Distribution of projects by rating')).toBeInTheDocument();
      expect(screen.getAllByText('All')).toHaveLength(3);
      expect(screen.getAllByText('Graduated')).toHaveLength(2);
      expect(screen.getAllByText('Incubating')).toHaveLength(2);
      expect(screen.getAllByText('Sandbox')).toHaveLength(2);
      expect(screen.getByText('Projects average score per category')).toBeInTheDocument();
      expect(screen.getByText('Repositories')).toBeInTheDocument();
      expect(screen.getByText('Percentage of repositories passing each check')).toBeInTheDocument();
    });

    it('renders component with empty stats', async () => {
      const mockStats = getMockStats('2');
      mocked(API).getStats.mockResolvedValue(mockStats);

      render(
        <AppContext.Provider value={{ ctx: mockCtx, dispatch: jest.fn() }}>
          <Router>
            <StatsView />
          </Router>
        </AppContext.Provider>
      );

      await waitFor(() => {
        expect(API.getStats).toHaveBeenCalledTimes(1);
      });

      const noData = await screen.findByRole('alert');
      expect(noData).toBeInTheDocument();
      expect(noData).toHaveTextContent('No Stats available for the moment');
    });
  });

  describe('when getStats call fails', () => {
    it('renders error message', async () => {
      mocked(API).getStats.mockRejectedValue(null);

      render(
        <AppContext.Provider value={{ ctx: mockCtx, dispatch: jest.fn() }}>
          <Router>
            <StatsView />
          </Router>
        </AppContext.Provider>
      );

      const noData = await screen.findByRole('alert');
      expect(noData).toBeInTheDocument();
      expect(noData).toHaveTextContent('An error occurred getting CLOMonitor stats, please try again later.');
    });
  });
});
