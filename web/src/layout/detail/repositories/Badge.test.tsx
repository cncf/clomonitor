import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { BrowserRouter as Router } from 'react-router-dom';

import Badge from './Badge';

const mockUseNavigate = jest.fn();

jest.mock('react-router-dom', () => ({
  ...(jest.requireActual('react-router-dom') as any),
  useNavigate: () => mockUseNavigate,
}));

const mockScrollIntoView = jest.fn();

const defaultProps = {
  linkTo: 'repo',
  scrollIntoView: mockScrollIntoView,
};

describe('Badge', () => {
  afterEach(() => {
    jest.resetAllMocks();
  });

  it('creates snapshot', () => {
    const { asFragment } = render(
      <Router>
        <Badge {...defaultProps} value={80} />
      </Router>
    );

    expect(asFragment()).toMatchSnapshot();
  });

  describe('Render', () => {
    it('renders badge', () => {
      render(
        <Router>
          <Badge {...defaultProps} value={80} />
        </Router>
      );

      expect(screen.getByText('80')).toBeInTheDocument();
    });

    it('renders badge with undefined value', () => {
      render(
        <Router>
          <Badge {...defaultProps} />
        </Router>
      );

      expect(screen.getByText('n/a')).toBeInTheDocument();
      expect(screen.queryByRole('button')).toBeNull();
    });

    it('clicks on anchor', () => {
      render(
        <Router>
          <Badge {...defaultProps} value={70} />
        </Router>
      );

      const btn = screen.getByRole('button', { name: 'Go from summary to section: repo' });
      userEvent.click(btn);

      expect(mockScrollIntoView).toHaveBeenCalledTimes(1);
      expect(mockScrollIntoView).toHaveBeenCalledWith('#repo');
      expect(mockUseNavigate).toHaveBeenCalledTimes(1);
      expect(mockUseNavigate).toHaveBeenCalledWith(
        {
          hash: 'repo',
          pathname: '/',
        },
        { state: null }
      );
    });
  });
});
