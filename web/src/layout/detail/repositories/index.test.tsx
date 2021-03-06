import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { BrowserRouter as Router } from 'react-router-dom';

import { Repository } from '../../../types';
import RepositoriesList from './index';
jest.mock('react-markdown', () => () => <div />);

const mockUseNavigate = jest.fn();

jest.mock('react-router-dom', () => ({
  ...(jest.requireActual('react-router-dom') as any),
  useNavigate: () => mockUseNavigate,
}));

const getRepositories = (fixtureId: string): Repository[] => {
  return require(`./__fixtures__/index/${fixtureId}.json`) as Repository[];
};

const mockScrollIntoView = jest.fn();

const defaultProps = {
  scrollIntoView: mockScrollIntoView,
};

describe('RepositoriesList', () => {
  afterEach(() => {
    jest.resetAllMocks();
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
  });
});
