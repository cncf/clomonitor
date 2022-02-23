import { render, screen } from '@testing-library/react';
import { BrowserRouter as Router } from 'react-router-dom';

import { ScoreKind, ScoreType } from '../../types';
import ScoreSummary from './ScoreSummary';

const defaultProps = {
  score: {
    [ScoreType.BestPractices]: 95,
    [ScoreType.Documentation]: 85,
    [ScoreType.Global]: 65,
    [ScoreType.License]: 80,
    score_kind: ScoreKind.Primary,
    [ScoreType.Security]: 0,
  },
  bigSize: false,
};

describe('ScoreSummary', () => {
  afterEach(() => {
    jest.resetAllMocks();
  });

  it('creates snapshot', () => {
    const { asFragment } = render(
      <Router>
        <ScoreSummary {...defaultProps} />
      </Router>
    );

    expect(asFragment()).toMatchSnapshot();
  });

  describe('Render', () => {
    it('renders component', () => {
      render(
        <Router>
          <ScoreSummary {...defaultProps} />
        </Router>
      );

      expect(screen.getByText('Documentation')).toBeInTheDocument();
      expect(screen.getByText('License')).toBeInTheDocument();
      expect(screen.getByText('Best Practices')).toBeInTheDocument();
      expect(screen.getByText('Security')).toBeInTheDocument();

      expect(screen.getByTestId('global-score')).toBeInTheDocument();
      expect(screen.getAllByTestId('line')).toHaveLength(4);
      expect(screen.getAllByTestId('peak')).toHaveLength(4);
    });

    it('renders correct classes when bigSize is true', () => {
      const { container } = render(
        <Router>
          <ScoreSummary {...defaultProps} bigSize />
        </Router>
      );

      expect(container.children[0]).toHaveClass('bigSize');
      expect(container.children[0].children[1]).toHaveClass('px-0 px-sm-3');
      expect(container.children[0].children[1].children[0]).toHaveClass('gx-4 gx-md-5');
      expect(screen.getByText('85')).toHaveClass('bigSize');
      expect(screen.getByTestId('global-score').parentNode).toHaveClass('mx-3');
    });
  });
});
