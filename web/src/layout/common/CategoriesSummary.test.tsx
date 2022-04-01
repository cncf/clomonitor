import { render, screen } from '@testing-library/react';
import { BrowserRouter as Router } from 'react-router-dom';

import { ScoreType } from '../../types';
import CategoriesSummary from './CategoriesSummary';

const defaultProps = {
  score: {
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
    jest.resetAllMocks();
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

      expect(screen.getByTestId('global-score')).toBeInTheDocument();
      expect(screen.getAllByTestId('line')).toHaveLength(5);
    });

    it('renders correct classes when bigSize is true', () => {
      const { container } = render(
        <Router>
          <CategoriesSummary {...defaultProps} bigSize />
        </Router>
      );

      expect(container.children[0]).toHaveClass('bigSize');
      expect(container.children[0].children[1]).toHaveClass('px-0 px-sm-3');
      expect(container.children[0].children[1].children[0]).toHaveClass('gx-4 gx-md-5');
      expect(screen.getByText('85')).toHaveClass('bigSize');
    });
  });
});
