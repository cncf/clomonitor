import { fireEvent, render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { createRequire } from 'module';
import { BrowserRouter as Router } from 'react-router-dom';
import { vi } from 'vitest';

import API from '../../api';
import { AppContext } from '../../context/AppContextProvider';
import { ScoreType, SortBy, SortDirection, Stats } from '../../types';
import StatsView from './index';

vi.mock('clo-ui/components/Timeline', () => ({
  Timeline: ({ setActiveDate }: { setActiveDate: (date?: string) => void }) => (
    <>
      <button type="button" onClick={() => setActiveDate('2023-02-08')}>
        Load historical stats snapshot
      </button>
      <button type="button" onClick={() => setActiveDate(undefined)}>
        Load current stats
      </button>
    </>
  ),
}));

const require = createRequire(import.meta.url);

const getMockStats = (fixtureId: string): Stats => {
  return require(`./__fixtures__/index/${fixtureId}.json`) as Stats;
};

const mockUseNavigate = vi.fn();

vi.mock('react-router-dom', async () => {
  const actual = await vi.importActual<typeof import('react-router-dom')>('react-router-dom');
  return {
    ...actual,
    useNavigate: () => mockUseNavigate,
  };
});

const mockCtx = {
  prefs: {
    search: { limit: 20, sort: { by: SortBy.Name, direction: SortDirection.ASC } },
    theme: { effective: 'light', configured: 'light' },
  },
};

const getStatsMock = vi.spyOn(API, 'getStats');
const getStatsSnapshotMock = vi.spyOn(API, 'getStatsSnapshot');
const getRepositoriesCsvMock = vi.spyOn(API, 'getRepositoriesCSV');

describe('StatsView', () => {
  afterEach(() => {
    getStatsMock.mockReset();
    getStatsSnapshotMock.mockReset();
    getRepositoriesCsvMock.mockReset();
    vi.clearAllMocks();
  });

  it('creates snapshot', async () => {
    const mockStats = getMockStats('1');
    getStatsMock.mockResolvedValue(mockStats);

    const { asFragment } = render(
      <AppContext.Provider value={{ ctx: mockCtx, dispatch: vi.fn() }}>
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
      getStatsMock.mockResolvedValue(mockStats);

      render(
        <AppContext.Provider value={{ ctx: mockCtx, dispatch: vi.fn() }}>
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
      expect(screen.getAllByText('Graduated')).toHaveLength(2);
      expect(screen.getAllByText('Incubating')).toHaveLength(2);
      expect(screen.getAllByText('Sandbox')).toHaveLength(2);
      expect(screen.getByText('Projects average score per category')).toBeInTheDocument();
      expect(screen.getByText('Repositories')).toBeInTheDocument();
      expect(screen.getByText('Percentage of repositories passing each check')).toBeInTheDocument();
      expect(screen.getAllByText('Agent Readiness').length).toBeGreaterThan(0);
    });

    it('renders current, historical, and current stats with missing, null, and zero Agent Readiness values', async () => {
      const currentStats = getMockStats('1');
      const historicalStats = structuredClone(currentStats);
      Object.values(historicalStats.projects.sections_average).forEach((average) => {
        delete average.agent_readiness;
      });
      delete historicalStats.repositories.passing_check![ScoreType.AgentReadiness];

      const currentStatsWithPartialValues = structuredClone(currentStats);
      currentStatsWithPartialValues.projects.sections_average.all.agent_readiness = 0;
      currentStatsWithPartialValues.projects.sections_average.sandbox.agent_readiness = null;

      getStatsMock.mockResolvedValueOnce(currentStats).mockResolvedValueOnce(currentStatsWithPartialValues);
      getStatsSnapshotMock.mockResolvedValueOnce(historicalStats);

      render(
        <AppContext.Provider value={{ ctx: mockCtx, dispatch: vi.fn() }}>
          <Router>
            <StatsView />
          </Router>
        </AppContext.Provider>
      );

      expect((await screen.findAllByText('Agent Readiness')).length).toBeGreaterThan(0);

      await userEvent.click(screen.getByRole('button', { name: 'Load historical stats snapshot' }));

      await waitFor(() => {
        expect(API.getStatsSnapshot).toHaveBeenCalledWith('2023-02-08', 'cncf');
      });
      expect(screen.queryAllByText('Agent Readiness')).toHaveLength(0);

      await userEvent.click(screen.getByRole('button', { name: 'Load current stats' }));

      await waitFor(() => {
        expect(API.getStats).toHaveBeenCalledTimes(2);
      });
      expect(screen.getAllByText('Agent Readiness').length).toBeGreaterThan(0);
      expect(
        screen
          .getAllByRole('progressbar', { name: 'Agent Readiness passed checks percentage' })
          .some((progressbar) => progressbar.getAttribute('aria-valuenow') === '0')
      ).toBe(true);
    });

    it('loads search page with correct parameters', async () => {
      const mockStats = getMockStats('1');
      getStatsMock.mockResolvedValue(mockStats);

      render(
        <AppContext.Provider value={{ ctx: mockCtx, dispatch: vi.fn() }}>
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

      const btn = screen.getByRole('button', { name: 'Search projects with passed Governance check' });
      await userEvent.click(btn);

      expect(mockUseNavigate).toHaveBeenCalledTimes(1);
      expect(mockUseNavigate).toHaveBeenCalledWith(
        {
          pathname: '/search',
          search: '?passing_check=governance&foundation=cncf&page=1',
        },
        { state: { resetScrollPosition: true } }
      );
    });

    it('loads search page with selected foundation', async () => {
      const mockStats = getMockStats('1');
      getStatsMock.mockResolvedValue(mockStats);

      render(
        <AppContext.Provider value={{ ctx: mockCtx, dispatch: vi.fn() }}>
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

      const select = screen.getByRole('combobox', { name: 'Foundation options select' });
      fireEvent.change(select, {
        target: { value: 'cncf' },
      });

      expect(screen.getByText('CNCF')).toBeInTheDocument();

      const btn = screen.getByRole('button', { name: 'Search projects with passed Governance check' });
      await userEvent.click(btn);

      expect(mockUseNavigate).toHaveBeenCalledTimes(1);
      expect(mockUseNavigate).toHaveBeenCalledWith(
        {
          pathname: '/search',
          search: '?passing_check=governance&foundation=cncf&page=1',
        },
        { state: { resetScrollPosition: true } }
      );
    });

    it('renders component with empty stats', async () => {
      const mockStats = getMockStats('2');
      getStatsMock.mockResolvedValue(mockStats);

      render(
        <AppContext.Provider value={{ ctx: mockCtx, dispatch: vi.fn() }}>
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

    it('renders component with Usage stats', async () => {
      const mockStats = getMockStats('3');
      getStatsMock.mockResolvedValue(mockStats);

      render(
        <AppContext.Provider value={{ ctx: mockCtx, dispatch: vi.fn() }}>
          <Router>
            <StatsView />
          </Router>
        </AppContext.Provider>
      );

      await waitFor(() => {
        expect(API.getStats).toHaveBeenCalledTimes(1);
      });

      expect(await screen.findByText('Usage')).toBeInTheDocument();
      expect(screen.getByText('Projects monthly views')).toBeInTheDocument();
      expect(screen.getByText('Projects daily views')).toBeInTheDocument();
    });
  });

  it('downloads repositories csv', async () => {
    const mockStats = getMockStats('1');
    getStatsMock.mockResolvedValue(mockStats);
    getRepositoriesCsvMock.mockResolvedValue('mock-csv');

    render(
      <AppContext.Provider value={{ ctx: mockCtx, dispatch: vi.fn() }}>
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

    const btn = screen.getByRole('button', { name: 'Download repositories CSV file' });
    await userEvent.click(btn);

    await waitFor(() => {
      expect(API.getRepositoriesCSV).toHaveBeenCalledTimes(1);
    });
  });

  describe('when getStats call fails', () => {
    it('renders error message', async () => {
      getStatsMock.mockRejectedValue(null);

      render(
        <AppContext.Provider value={{ ctx: mockCtx, dispatch: vi.fn() }}>
          <Router>
            <StatsView />
          </Router>
        </AppContext.Provider>
      );

      const noData = await screen.findByRole('alert');
      expect(noData).toBeInTheDocument();
      expect(noData).toHaveTextContent('An error occurred getting CLOMonitor stats.');
      expect(noData).toHaveTextContent('Please try again later.');
    });
  });
});
