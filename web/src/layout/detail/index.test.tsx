import { render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { createRequire } from 'module';
import { vi } from 'vitest';

import API from '../../api';
import { ProjectDetail, ScoreType } from '../../types';
import Detail from './index';
vi.mock('../../utils/updateMetaIndex');
vi.mock('react-markdown', () => ({
  __esModule: true,
  default: () => <div />,
}));
vi.mock('rehype-external-links', () => ({
  __esModule: true,
  default: () => <></>,
}));

vi.mock('clo-ui/components/Timeline', () => ({
  Timeline: ({ setActiveDate }: { setActiveDate: (date?: string) => void }) => (
    <>
      <button type="button" onClick={() => setActiveDate('2023-02-08')}>
        Load historical snapshot
      </button>
      <button type="button" onClick={() => setActiveDate(undefined)}>
        Load current snapshot
      </button>
    </>
  ),
}));

vi.mock('date-fns', async () => {
  const actual = await vi.importActual<typeof import('date-fns')>('date-fns');
  return {
    ...actual,
    formatDistanceToNowStrict: () => '4 years ago',
  };
});

const mockUseNavigate = vi.fn();
const mockUseParams = vi.fn();
const mockUseLocation = vi.fn();

vi.mock('react-router-dom', async () => {
  const actual = await vi.importActual<typeof import('react-router-dom')>('react-router-dom');
  return {
    ...actual,
    useNavigate: () => mockUseNavigate,
    useParams: () => mockUseParams(),
    useLocation: () => mockUseLocation(),
  };
});

const { BrowserRouter: Router } = await import('react-router-dom');

const require = createRequire(import.meta.url);

const getMockDetail = (fixtureId: string): ProjectDetail => {
  return require(`./__fixtures__/index/${fixtureId}.json`) as ProjectDetail;
};

const path = {
  pathname: '/projects/cncf/artifact-hub/artifact-hub',
  search: '',
  hash: '',
  state: { currentSearch: '?maturity=sandbox&rating=a&page=1' },
  key: 'key',
};

const defaultProps = {
  setInvisibleFooter: vi.fn(),
};

describe('Project detail index', () => {
  const getProjectDetailMock = vi.spyOn(API, 'getProjectDetail');
  const getProjectSnapshotMock = vi.spyOn(API, 'getProjectSnapshot');

  beforeEach(() => {
    mockUseParams.mockReturnValue({ project: 'proj', foundation: 'cncf' });
    mockUseLocation.mockReturnValue(path);
    mockUseNavigate.mockImplementation(() => undefined);
  });

  afterEach(() => {
    getProjectDetailMock.mockReset();
    getProjectSnapshotMock.mockReset();
    mockUseParams.mockReset();
    mockUseLocation.mockReset();
    mockUseNavigate.mockReset();
  });

  it('creates snapshot', async () => {
    const mockProject = getMockDetail('1');
    getProjectDetailMock.mockResolvedValue(mockProject);

    const { asFragment } = render(
      <Router>
        <Detail {...defaultProps} />
      </Router>
    );

    await waitFor(() => {
      expect(API.getProjectDetail).toHaveBeenCalledTimes(1);
      expect(asFragment()).toMatchSnapshot();
    });
  });

  describe('Render', () => {
    it('renders component', async () => {
      const mockProject = getMockDetail('1');
      getProjectDetailMock.mockResolvedValue(mockProject);

      render(
        <Router>
          <Detail {...defaultProps} />
        </Router>
      );

      await waitFor(() => {
        expect(API.getProjectDetail).toHaveBeenCalledTimes(1);
        expect(API.getProjectDetail).toHaveBeenCalledWith('proj', 'cncf');
      });

      expect(screen.getByAltText('Artifact Hub logo')).toBeInTheDocument();
      expect(screen.getByText('Artifact Hub')).toBeInTheDocument();
      expect(
        screen.getByText(
          'Artifact Hub is a web-based application that enables finding, installing, and publishing packages and configurations for CNCF projects.'
        )
      ).toBeInTheDocument();
      expect(screen.getByText('Sandbox')).toBeInTheDocument();
      expect(screen.getByText('app definition')).toBeInTheDocument();
      expect(screen.getByText('CNCF')).toBeInTheDocument();
      expect(await screen.findByRole('link', { name: 'Repository link' })).toBeInTheDocument();
      expect(screen.getByText('Accepted:')).toBeInTheDocument();
      expect(screen.getAllByText('23rd June 2020').length).toBeGreaterThan(0);
      expect(screen.getAllByTestId('dropdown-btn')).toHaveLength(2);
      expect(screen.getAllByText('Agent Readiness').length).toBeGreaterThan(0);
    });

    it('renders current, historical, and current Agent Readiness states with missing, null, and zero scores', async () => {
      const currentProject = getMockDetail('1');
      const historicalProject = structuredClone(currentProject);
      delete historicalProject.score[ScoreType.AgentReadiness];
      delete historicalProject.repositories[0].score![ScoreType.AgentReadiness];
      delete historicalProject.repositories[0].report!.data.agent_readiness;

      const currentProjectWithPartialScores = structuredClone(currentProject);
      currentProjectWithPartialScores.score[ScoreType.AgentReadiness] = null;
      currentProjectWithPartialScores.repositories[0].score![ScoreType.AgentReadiness] = 0;

      getProjectDetailMock.mockResolvedValueOnce(currentProject).mockResolvedValueOnce(currentProjectWithPartialScores);
      getProjectSnapshotMock.mockResolvedValueOnce(historicalProject);

      render(
        <Router>
          <Detail {...defaultProps} />
        </Router>
      );

      expect((await screen.findAllByText('Agent Readiness')).length).toBeGreaterThan(0);

      await userEvent.click(screen.getByRole('button', { name: 'Load historical snapshot' }));

      await waitFor(() => {
        expect(API.getProjectSnapshot).toHaveBeenCalledWith('proj', 'cncf', '2023-02-08');
      });
      expect(screen.getAllByText('Agent Readiness').length).toBeGreaterThan(0);
      expect(screen.getAllByText('n/a').length).toBeGreaterThan(0);

      await userEvent.click(screen.getByRole('button', { name: 'Load current snapshot' }));

      await waitFor(() => {
        expect(API.getProjectDetail).toHaveBeenCalledTimes(2);
      });
      expect((await screen.findAllByText('Agent Readiness')).length).toBeGreaterThan(0);
      expect(
        await screen.findByRole('progressbar', { name: 'Agent Readiness score for artifact-hub' })
      ).toHaveAttribute('aria-valuenow', '0');
    });

    it('renders Back to results', async () => {
      mockUseLocation.mockReturnValue({
        ...path,
        state: { currentSearch: '?maturity=sandbox&rating=a&page=1' },
      });

      const mockProject = getMockDetail('1');
      getProjectDetailMock.mockResolvedValue(mockProject);

      render(
        <Router>
          <Detail {...defaultProps} />
        </Router>
      );

      await waitFor(() => {
        expect(API.getProjectDetail).toHaveBeenCalledTimes(1);
        expect(API.getProjectDetail).toHaveBeenCalledWith('proj', 'cncf');
      });

      const backBtn = screen.getByRole('button', { name: 'Back to results' });
      expect(backBtn).toBeInTheDocument();

      await userEvent.click(backBtn);

      expect(mockUseNavigate).toHaveBeenCalledTimes(1);
      expect(mockUseNavigate).toHaveBeenCalledWith('/search?maturity=sandbox&rating=a&page=1');
    });

    it('renders placeholder when no data', async () => {
      getProjectDetailMock.mockRejectedValue('');

      render(
        <Router>
          <Detail {...defaultProps} />
        </Router>
      );

      await waitFor(() => {
        expect(API.getProjectDetail).toHaveBeenCalledTimes(1);
        expect(API.getProjectDetail).toHaveBeenCalledWith('proj', 'cncf');
      });

      expect(await screen.findByText('The requested project was not found.')).toBeInTheDocument();
      expect(screen.getByText('The project you are looking for may have been deleted.')).toBeInTheDocument();
    });
  });
});
