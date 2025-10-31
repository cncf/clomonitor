import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { BrowserRouter as Router } from 'react-router-dom';
import { vi } from 'vitest';

import RepositoryDropdown from './RepositoryDropdown';

const { mockUseParams } = vi.hoisted(() => ({
  mockUseParams: vi.fn(),
}));

vi.mock('react-router-dom', async () => {
  const actual = await vi.importActual<typeof import('react-router-dom')>('react-router-dom');
  return {
    ...actual,
    useParams: mockUseParams,
  };
});

const defaultProps = {
  repoName: 'repo',
};

describe('RepositoryDropdown', () => {
  beforeEach(() => {
    mockUseParams.mockReturnValue({ project: 'proj', foundation: 'cncf' });
  });

  afterEach(() => {
    mockUseParams.mockReset();
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
