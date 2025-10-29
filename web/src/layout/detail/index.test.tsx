import { createRequire } from 'module';

import { render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { vi } from 'vitest';

import API from '../../api';
import { ProjectDetail } from '../../types';
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
  Timeline: () => <>Timeline</>,
}));

vi.mock('moment', async () => {
  const actual = await vi.importActual<typeof import('moment')>('moment');
  return {
    ...actual,
    unix: () => ({
      fromNow: () => '3 days ago',
      format: () => '23rd June 2020',
    }),
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

  beforeEach(() => {
    mockUseParams.mockReturnValue({ project: 'proj', foundation: 'cncf' });
    mockUseLocation.mockReturnValue(path);
    mockUseNavigate.mockImplementation(() => undefined);
  });

  afterEach(() => {
    getProjectDetailMock.mockReset();
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
