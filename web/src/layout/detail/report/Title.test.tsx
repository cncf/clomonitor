import { render, screen } from '@testing-library/react';
import { vi } from 'vitest';

import Title from './Title';

const defaultProps = {
  title: 'title',
  icon: <>icon</>,
};

describe('Title', () => {
  afterEach(() => {
    vi.resetAllMocks();
  });

  it('creates snapshot', () => {
    const { asFragment } = render(<Title {...defaultProps} />);

    expect(asFragment()).toMatchSnapshot();
  });

  describe('Render', () => {
    it('renders option', () => {
      render(<Title {...defaultProps} />);

      expect(screen.getByText('title')).toBeInTheDocument();
      expect(screen.getByText('icon')).toBeInTheDocument();
    });
  });
});
