import { render, screen } from '@testing-library/react';

import QualityDot from './QualityDot';

describe('QualityDot', () => {
  afterEach(() => {
    jest.resetAllMocks();
  });

  it('creates snapshot', () => {
    const { asFragment } = render(<QualityDot level={1} />);
    expect(asFragment()).toMatchSnapshot();
  });

  it('renders proper content', () => {
    render(<QualityDot level={1} />);

    expect(screen.getByTestId('quality-dot')).toBeInTheDocument();
    expect(screen.getByTestId('quality-dot')).toHaveClass('level1');
  });

  it('renders level 3 dot', () => {
    render(<QualityDot level={3} />);

    expect(screen.getByTestId('quality-dot')).toHaveClass('level3');
  });
});
