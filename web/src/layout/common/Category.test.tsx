import { render, screen } from '@testing-library/react';

import Category from './Category';

const defaultProps = {
  value: 80,
  name: 'Documentation',
  shortName: 'Docs',
};

describe('Category', () => {
  afterEach(() => {
    jest.resetAllMocks();
  });

  it('creates snapshot', () => {
    const { asFragment } = render(<Category {...defaultProps} />);
    expect(asFragment()).toMatchSnapshot();
  });

  it('renders proper content', () => {
    render(<Category {...defaultProps} />);
    expect(screen.getByText('Documentation')).toBeInTheDocument();
    expect(screen.getByText('Documentation')).toHaveClass('d-none d-md-inline-block');
    expect(screen.getByText('Docs')).toBeInTheDocument();
    expect(screen.getByText('Docs')).toHaveClass('d-inline-block d-md-none');
    expect(screen.getByText('80')).toBeInTheDocument();

    const line = screen.getByTestId('line');
    expect(line).toBeInTheDocument();
    expect(line).toHaveStyle('width: calc(80% - 5px)');

    expect(screen.getByTestId('peak')).toBeInTheDocument();
  });

  it('does not render peak when value is 100', () => {
    render(<Category {...defaultProps} value={100} />);
    expect(screen.queryByTestId('peak')).toBeNull();
  });
});
