import { render, screen } from '@testing-library/react';

import Title from './Title';

const defaultProps = {
  title: 'title',
  icon: <>icon</>,
};

describe('Title', () => {
  afterEach(() => {
    jest.resetAllMocks();
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
