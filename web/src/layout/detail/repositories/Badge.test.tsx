import { render, screen } from '@testing-library/react';

import Badge from './Badge';

describe('Badge', () => {
  afterEach(() => {
    jest.resetAllMocks();
  });

  it('creates snapshot', () => {
    const { asFragment } = render(<Badge value={80} />);

    expect(asFragment()).toMatchSnapshot();
  });

  describe('Render', () => {
    it('renders badge', () => {
      render(<Badge value={80} />);

      expect(screen.getByText('80')).toBeInTheDocument();
    });

    it('renders badge with undefined value', () => {
      render(<Badge />);

      expect(screen.getByText('n/a')).toBeInTheDocument();
    });
  });
});
