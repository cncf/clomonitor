import { render, screen } from '@testing-library/react';

import { Foundation } from '../../../types';
import FoundationBadge from './FoundationBadge';

describe('FoundationBadge', () => {
  afterEach(() => {
    jest.resetAllMocks();
  });

  it('creates snapshot', () => {
    const { asFragment } = render(<FoundationBadge foundation={Foundation.cncf} />);
    expect(asFragment()).toMatchSnapshot();
  });

  it('renders foundation CNCF', () => {
    render(<FoundationBadge foundation={Foundation.cncf} />);
    expect(screen.getByText('CNCF')).toBeInTheDocument();
  });

  it('renders foundation LFAI', () => {
    render(<FoundationBadge foundation={Foundation.lfaidata} />);
    expect(screen.getByText('LF AI & Data')).toBeInTheDocument();
  });
});
