import { createRequire } from 'module';

import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { BrowserRouter as Router } from 'react-router-dom';
import { vi } from 'vitest';

import { Repository } from '../../../types';
import RepositoriesList from './index';
vi.mock('react-markdown', () => ({
  __esModule: true,
  default: () => <div />,
}));
vi.mock('rehype-external-links', () => ({
  __esModule: true,
  default: () => <></>,
}));

const { mockUseNavigate, mockUseParams } = vi.hoisted(() => ({
  mockUseNavigate: vi.fn(),
  mockUseParams: vi.fn(),
}));

vi.mock('react-router-dom', async () => {
  const actual = await vi.importActual<typeof import('react-router-dom')>('react-router-dom');
  return {
    ...actual,
    useNavigate: () => mockUseNavigate,
    useParams: mockUseParams,
  };
});

const require = createRequire(import.meta.url);

const getRepositories = (fixtureId: string): Repository[] => {
  return require(`./__fixtures__/index/${fixtureId}.json`) as Repository[];
};

const mockScrollIntoView = vi.fn();

const defaultProps = {
  isSnapshotVisible: false,
  scrollIntoView: mockScrollIntoView,
};

describe('RepositoriesList', () => {
  beforeEach(() => {
    mockUseParams.mockReturnValue({ project: 'proj', foundation: 'cncf' });
  });

  afterEach(() => {
    mockUseNavigate.mockReset();
    mockUseParams.mockReset();
    vi.clearAllMocks();
  });

  it('creates snapshot', () => {
    const repositories = getRepositories('1');
    const { asFragment } = render(
      <Router>
        <RepositoriesList {...defaultProps} repositories={repositories} />
      </Router>
    );
    expect(asFragment()).toMatchSnapshot();
  });

  describe('Render', () => {
    it('renders component', () => {
      const repositories = getRepositories('1');
      render(
        <Router>
          <RepositoriesList {...defaultProps} repositories={repositories} />
        </Router>
      );

      expect(screen.getByText('Repositories')).toBeInTheDocument();
      expect(screen.getByTestId('repositories-summary')).toBeInTheDocument();
      expect(screen.getAllByTestId('repository-info')).toHaveLength(6);
      expect(screen.getAllByTestId('dropdown-btn')).toHaveLength(6);
      const anchorBtns = screen.getAllByRole('button', { name: /Link to anchor/i });
      expect(anchorBtns).toHaveLength(26);

      // Sorted repos
      expect(anchorBtns[0]).toHaveAttribute('aria-label', 'Link to anchor spec');
      expect(anchorBtns[6]).toHaveAttribute('aria-label', 'Link to anchor sdk-go');
      expect(anchorBtns[10]).toHaveAttribute('aria-label', 'Link to anchor sdk-javascript');
      expect(anchorBtns[14]).toHaveAttribute('aria-label', 'Link to anchor sdk-csharp');
      expect(anchorBtns[18]).toHaveAttribute('aria-label', 'Link to anchor sdk-java');
      expect(anchorBtns[22]).toHaveAttribute('aria-label', 'Link to anchor sdk-python');
    });

    it('renders component when one repo fails', () => {
      const repositories = getRepositories('2');
      render(
        <Router>
          <RepositoriesList {...defaultProps} repositories={repositories} />
        </Router>
      );

      expect(screen.getByText('error running dco check')).toBeInTheDocument();
      expect(screen.getByText('-')).toBeInTheDocument();
    });

    it('does not render repository without report', () => {
      const repositories = getRepositories('3');
      render(
        <Router>
          <RepositoriesList {...defaultProps} repositories={repositories} />
        </Router>
      );

      expect(screen.queryByText('grpc.io')).toBeNull();
    });

    it('clicks anchor link', async () => {
      const repositories = getRepositories('1');
      render(
        <Router>
          <RepositoriesList {...defaultProps} repositories={repositories} />
        </Router>
      );

      const anchors = screen.getAllByRole('button', { name: 'Link to anchor sdk-go' });
      await userEvent.click(anchors[0]);

      expect(mockScrollIntoView).toHaveBeenCalledTimes(1);
      expect(mockScrollIntoView).toHaveBeenCalledWith('#sdk-go');
      expect(mockUseNavigate).toHaveBeenCalledTimes(1);
      expect(mockUseNavigate).toHaveBeenCalledWith(
        {
          hash: 'sdk-go',
          pathname: '/',
        },
        { state: null }
      );
    });

    it('does not render dropdown when snapshot is visible', async () => {
      const repositories = getRepositories('1');
      render(
        <Router>
          <RepositoriesList {...defaultProps} repositories={repositories} isSnapshotVisible />
        </Router>
      );

      expect(screen.getByText('Repositories')).toBeInTheDocument();
      expect(screen.queryByTestId('dropdown-btn')).toBeNull();
    });
  });
});
