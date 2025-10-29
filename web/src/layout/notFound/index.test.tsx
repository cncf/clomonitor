import { render, screen } from '@testing-library/react';
import { BrowserRouter as Router } from 'react-router-dom';
import { vi } from 'vitest';

import NotFound from './index';

describe('NotFound', () => {
  afterEach(() => {
    vi.resetAllMocks();
  });

  it('creates snapshot', () => {
    const { asFragment } = render(
      <Router>
        <NotFound />
      </Router>
    );

    expect(asFragment()).toMatchSnapshot();
  });

  describe('Render', () => {
    it('renders NotFound', () => {
      render(
        <Router>
          <NotFound />
        </Router>
      );

      expect(screen.getByText('Error 404 - Page Not Found')).toBeInTheDocument();
      expect(screen.getByText("The page you were looking for wasn't found")).toBeInTheDocument();

      const link = screen.getByRole('link');
      expect(link).toBeInTheDocument();
      expect(link).toHaveAttribute('href', '/search?page=1');
    });
  });
});
