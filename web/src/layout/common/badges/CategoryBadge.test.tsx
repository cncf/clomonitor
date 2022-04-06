import { render, screen } from '@testing-library/react';

import CategoryBadge from './CategoryBadge';

describe('CategoryBadge', () => {
  afterEach(() => {
    jest.resetAllMocks();
  });

  it('creates snapshot', () => {
    const { asFragment } = render(<CategoryBadge category="app definition" />);
    expect(asFragment()).toMatchSnapshot();
  });

  it('renders category', () => {
    render(<CategoryBadge category="app definition" />);
    expect(screen.getByText('app definition')).toBeInTheDocument();
  });
});
