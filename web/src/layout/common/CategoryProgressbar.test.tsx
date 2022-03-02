import { render, screen } from '@testing-library/react';

import CategoryProgressbar from './CategoryProgressbar';

const defaultProps = {
  value: 80,
  name: 'Documentation',
};

describe('CategoryProgressbar', () => {
  afterEach(() => {
    jest.resetAllMocks();
  });

  it('creates snapshot', () => {
    const { asFragment } = render(<CategoryProgressbar {...defaultProps} />);
    expect(asFragment()).toMatchSnapshot();
  });

  it('renders proper content', () => {
    render(<CategoryProgressbar {...defaultProps} />);
    expect(screen.getByText('Documentation')).toBeInTheDocument();
    expect(screen.getByText('80')).toBeInTheDocument();

    const line = screen.getByTestId('line');
    expect(line).toBeInTheDocument();
    expect(line).toHaveStyle('width: calc(80% - 5px)');

    expect(screen.getByTestId('peak')).toBeInTheDocument();
  });
});
