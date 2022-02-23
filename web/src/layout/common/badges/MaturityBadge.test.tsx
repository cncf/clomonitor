import { render, screen } from '@testing-library/react';

import { Maturity } from '../../../types';
import MaturityBadge from './MaturityBadge';

describe('MaturityBadge', () => {
  afterEach(() => {
    jest.resetAllMocks();
  });

  it('creates snapshot', () => {
    const { asFragment } = render(<MaturityBadge maturityLevel={Maturity.Graduated} />);
    expect(asFragment()).toMatchSnapshot();
  });

  it('renders category 0', () => {
    render(<MaturityBadge maturityLevel={Maturity.Graduated} />);
    expect(screen.getByText('Graduated')).toBeInTheDocument();
  });

  it('renders category 1', () => {
    render(<MaturityBadge maturityLevel={Maturity.Incubating} />);
    expect(screen.getByText('Incubating')).toBeInTheDocument();
  });

  it('renders category 2', () => {
    render(<MaturityBadge maturityLevel={Maturity.Sandbox} />);
    expect(screen.getByText('Sandbox')).toBeInTheDocument();
  });
});
