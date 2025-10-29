import { render, screen } from '@testing-library/react';
import { vi } from 'vitest';

import QualityDot from './QualityDot';
import styles from './QualityDot.module.css';

describe('QualityDot', () => {
  afterEach(() => {
    vi.resetAllMocks();
  });

  it('creates snapshot', () => {
    const { asFragment } = render(<QualityDot level={1} />);
    expect(asFragment()).toMatchSnapshot();
  });

  it('renders proper content', () => {
    render(<QualityDot level={1} />);

    expect(screen.getByTestId('quality-dot')).toBeInTheDocument();
    expect(screen.getByTestId('quality-dot')).toHaveClass(styles.level1);
  });

  it('renders level 3 dot', () => {
    render(<QualityDot level={3} />);

    expect(screen.getByTestId('quality-dot')).toHaveClass(styles.level3);
  });
});
