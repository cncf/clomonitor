import { render, screen } from '@testing-library/react';
import { vi } from 'vitest';

import Average from './Average';

const defaultProps = {
  data: { documentation: 85, license: 79, best_practices: 57, security: 53, legal: 40, global: 69 },
  title: 'Sandbox',
};

describe('Average', () => {
  afterEach(() => {
    vi.resetAllMocks();
  });

  it('creates snapshot', () => {
    const { asFragment } = render(<Average {...defaultProps} />);

    expect(asFragment()).toMatchSnapshot();
  });

  describe('Render', () => {
    it('renders component', () => {
      render(<Average {...defaultProps} />);

      expect(screen.getByText('Sandbox')).toBeInTheDocument();
      expect(screen.getByText('Documentation')).toBeInTheDocument();
      expect(screen.getByText('85%')).toBeInTheDocument();
      expect(screen.getByText('License')).toBeInTheDocument();
      expect(screen.getByText('79%')).toBeInTheDocument();
      expect(screen.getByText('Best Practices')).toBeInTheDocument();
      expect(screen.getByText('57%')).toBeInTheDocument();
      expect(screen.getByText('Security')).toBeInTheDocument();
      expect(screen.getByText('53%')).toBeInTheDocument();
      expect(screen.getByText('Legal')).toBeInTheDocument();
      expect(screen.getByText('40%')).toBeInTheDocument();

      expect(screen.getAllByRole('progressbar')).toHaveLength(5);
    });
  });
});
