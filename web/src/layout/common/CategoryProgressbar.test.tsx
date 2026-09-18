import { render, screen } from '@testing-library/react';
import { BrowserRouter as Router } from 'react-router-dom';
import { vi } from 'vitest';

import CategoryProgressbar from './CategoryProgressbar';

const defaultProps = {
  value: 80,
  name: 'Documentation',
};

describe('CategoryProgressbar', () => {
  afterEach(() => {
    vi.resetAllMocks();
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

  it('renders short and full names when a short name is provided', () => {
    render(
      <Router>
        <CategoryProgressbar {...defaultProps} name="Agent Readiness" shortName="Agents" />
      </Router>
    );

    expect(screen.getByText('Agents')).toHaveClass('d-none', 'd-lg-block', 'd-xxl-none');
    expect(screen.getByText('Agent Readiness')).toHaveClass('d-lg-none', 'd-xxl-block');
  });

  it('renders short and full names inside the section link', () => {
    render(
      <Router>
        <CategoryProgressbar {...defaultProps} name="Agent Readiness" shortName="Agents" linkTo="repo_agent" />
      </Router>
    );

    const link = screen.getByRole('button', { name: 'Go from summary to section: repo_agent' });
    expect(link).toHaveTextContent('Agents');
    expect(link).toHaveTextContent('Agent Readiness');
  });
});
