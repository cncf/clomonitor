import { render, screen } from '@testing-library/react';

import RoundScore from './RoundScore';

describe('RoundScore', () => {
  afterEach(() => {
    jest.resetAllMocks();
  });

  it('creates snapshot', () => {
    const { asFragment } = render(<RoundScore score={80} />);
    expect(asFragment()).toMatchSnapshot();
  });

  it('renders proper content', () => {
    render(<RoundScore score={80} />);

    expect(screen.getByTestId('global-score')).toBeInTheDocument();
    expect(screen.getByText('80')).toBeInTheDocument();
  });
});
