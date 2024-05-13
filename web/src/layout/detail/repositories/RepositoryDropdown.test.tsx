import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import ReactRouter, { BrowserRouter as Router } from 'react-router-dom';

import RepositoryDropdown from './RepositoryDropdown';

jest.mock('react-router-dom', () => ({
  ...(jest.requireActual('react-router-dom') as object),
  useParams: jest.fn(),
}));

const defaultProps = {
  repoName: 'repo',
};

describe('RepositoryDropdown', () => {
  beforeEach(() => {
    jest.spyOn(ReactRouter, 'useParams').mockReturnValue({ project: 'proj', foundation: 'cncf' });
  });

  afterEach(() => {
    jest.resetAllMocks();
  });

  it('creates snapshot', () => {
    const { asFragment } = render(
      <Router>
        <RepositoryDropdown {...defaultProps} />
      </Router>
    );

    expect(asFragment()).toMatchSnapshot();
  });

  describe('Render', () => {
    it('renders properly', () => {
      render(
        <Router>
          <RepositoryDropdown {...defaultProps} />
        </Router>
      );

      expect(screen.getByTestId('dropdown-btn')).toBeInTheDocument();
    });

    it('opens dropdown', async () => {
      render(
        <Router>
          <RepositoryDropdown {...defaultProps} />
        </Router>
      );

      const btn = screen.getByTestId('dropdown-btn');
      await userEvent.click(btn);

      expect(await screen.findByRole('complementary')).toBeInTheDocument();
      expect(screen.getByRole('button', { name: 'Open repository report' })).toBeInTheDocument();
    });
  });
});
