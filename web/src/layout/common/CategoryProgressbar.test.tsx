import { render, screen } from '@testing-library/react';
import { BrowserRouter as Router } from 'react-router-dom';

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
    const { asFragment } = render(
      <Router>
        <CategoryProgressbar {...defaultProps} />
      </Router>
    );
    expect(asFragment()).toMatchSnapshot();
  });

  it('renders proper content', () => {
    render(
      <Router>
        <CategoryProgressbar {...defaultProps} />
      </Router>
    );
    expect(screen.getByText('Documentation')).toBeInTheDocument();
    expect(screen.getByText('80')).toBeInTheDocument();

    const line = screen.getByTestId('line');
    expect(line).toBeInTheDocument();
    expect(line).toHaveStyle('width: 80%');
  });
});
