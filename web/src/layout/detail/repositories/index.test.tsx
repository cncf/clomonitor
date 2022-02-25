import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { BrowserRouter as Router } from 'react-router-dom';

import { Repository } from '../../../types';
import RepositoriesList from './index';

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
      expect(screen.getByTestId('primary-icon')).toBeInTheDocument();
      expect(screen.getAllByTestId('repository-info')).toHaveLength(6);
      const anchorBtns = screen.getAllByRole('button', { name: /Anchor to/i });
      expect(anchorBtns).toHaveLength(6 * 2);

      // Sorted repos
      expect(anchorBtns[0]).toHaveAttribute('aria-label', 'Anchor to spec');
      expect(anchorBtns[2]).toHaveAttribute('aria-label', 'Anchor to sdk-go');
      expect(anchorBtns[4]).toHaveAttribute('aria-label', 'Anchor to sdk-javascript');
      expect(anchorBtns[6]).toHaveAttribute('aria-label', 'Anchor to sdk-csharp');
      expect(anchorBtns[8]).toHaveAttribute('aria-label', 'Anchor to sdk-java');
      expect(anchorBtns[10]).toHaveAttribute('aria-label', 'Anchor to sdk-python');
    });

    it('clicks anchor link', () => {
      const repositories = getRepositories('1');
      render(
        <Router>
          <RepositoriesList {...defaultProps} repositories={repositories} />
        </Router>
      );

      const anchors = screen.getAllByRole('button', { name: 'Anchor to sdk-go' });
      userEvent.click(anchors[0]);

      expect(mockScrollIntoView).toHaveBeenCalledTimes(1);
      expect(mockScrollIntoView).toHaveBeenCalledWith('#sdk-go');
      expect(mockUseNavigate).toHaveBeenCalledTimes(1);
      expect(mockUseNavigate).toHaveBeenCalledWith(
        {
          hash: 'sdk-go',
          pathname: '/',
        },
        { replace: true, state: null }
      );
    });
  });
});
