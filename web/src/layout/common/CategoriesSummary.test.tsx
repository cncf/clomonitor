import { render, screen } from '@testing-library/react';
import { BrowserRouter as Router } from 'react-router-dom';
import { vi } from 'vitest';

import { ScoreType } from '../../types';
import CategoriesSummary from './CategoriesSummary';
import styles from './CategoriesSummary.module.css';

const defaultProps = {
  score: {
    [ScoreType.AgentReadiness]: 57,
    [ScoreType.BestPractices]: 95,
    [ScoreType.Documentation]: 85,
    [ScoreType.Global]: 65,
    [ScoreType.License]: 80,
    [ScoreType.Security]: 0,
    [ScoreType.Legal]: 75,
  },
  bigSize: false,
};

describe('CategoriesSummary', () => {
  afterEach(() => {
    vi.resetAllMocks();
  });

  it('creates snapshot', () => {
    const { asFragment } = render(
      <Router>
        <CategoriesSummary {...defaultProps} />
      </Router>
    );

    expect(asFragment()).toMatchSnapshot();
  });

  describe('Render', () => {
    it('renders component', () => {
      render(
        <Router>
          <CategoriesSummary {...defaultProps} />
        </Router>
      );

      expect(screen.getByText('Documentation')).toBeInTheDocument();
      expect(screen.getByText('License')).toBeInTheDocument();
      expect(screen.getByText('Best Practices')).toBeInTheDocument();
      expect(screen.getByText('Security')).toBeInTheDocument();
      expect(screen.getByText('Legal')).toBeInTheDocument();
      expect(screen.getByText('Agent Readiness')).toBeInTheDocument();

      expect(screen.getByTestId('global-score')).toBeInTheDocument();
      expect(screen.getAllByTestId('line')).toHaveLength(6);
    });

    it('shows n/a for missing Agent Readiness scores', () => {
      render(
        <Router>
          <CategoriesSummary
            {...defaultProps}
            score={{ ...defaultProps.score, [ScoreType.AgentReadiness]: undefined }}
          />
        </Router>
      );

      expect(screen.getByText('Agent Readiness')).toBeInTheDocument();
      expect(screen.getByText('n/a')).toBeInTheDocument();
      expect(screen.getAllByTestId('line')).toHaveLength(5);
    });

    it('uses the short Agent Readiness name only in compact mode', () => {
      const { rerender } = render(
        <Router>
          <CategoriesSummary {...defaultProps} />
        </Router>
      );

      expect(screen.getByText('Agents')).toBeInTheDocument();

      rerender(
        <Router>
          <CategoriesSummary {...defaultProps} bigSize />
        </Router>
      );

      expect(screen.queryByText('Agents')).toBeNull();
      expect(screen.getByText('Agent Readiness')).toBeInTheDocument();
    });

    it('renders correct classes when bigSize is true', () => {
      const { container } = render(
        <Router>
          <CategoriesSummary {...defaultProps} bigSize />
        </Router>
      );

      expect(container.children[0]).toHaveClass(styles.bigSize);
      expect(container.children[0].children[1]).toHaveClass('px-0 px-sm-3');
      expect(container.children[0].children[1].children[0]).toHaveClass('gx-4 gx-md-5');
      expect(screen.getByText('85').className).toContain('bigSize');
    });
  });
});
