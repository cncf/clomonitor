import { render, screen } from '@testing-library/react';

import { Maturity } from '../../../types';
import MaturityBadge from './MaturityBadge';

describe('MaturityBadge', () => {
  afterEach(() => {
    jest.resetAllMocks();
  });

  it('creates snapshot', () => {
    const { asFragment } = render(<MaturityBadge maturityLevel={Maturity.graduated} />);
    expect(asFragment()).toMatchSnapshot();
  });

  it('renders category graduated', () => {
    render(<MaturityBadge maturityLevel={Maturity.graduated} />);
    expect(screen.getByText('graduated')).toBeInTheDocument();
  });

  it('renders category incubating', () => {
    render(<MaturityBadge maturityLevel={Maturity.incubating} />);
    expect(screen.getByText('incubating')).toBeInTheDocument();
  });

  it('renders category sandbox', () => {
    render(<MaturityBadge maturityLevel={Maturity.sandbox} />);
    expect(screen.getByText('sandbox')).toBeInTheDocument();
  });
});
