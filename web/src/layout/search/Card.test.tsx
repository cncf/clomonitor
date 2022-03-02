import { render, screen } from '@testing-library/react';
import { BrowserRouter as Router } from 'react-router-dom';

import { RepositoryKind, ScoreKind } from '../../types';
import Card from './Card';

jest.mock('moment', () => ({
  ...(jest.requireActual('moment') as {}),
  unix: () => ({
    fromNow: () => '3 days ago',
  }),
}));

const mockSaveScrollPosition = jest.fn();

const defaultProps = {
  project: {
    category_id: 0,
    description:
      'Artifact Hub is a web-based application that enables finding, installing, and publishing packages and configurations for CNCF projects.',
    devstats_url: 'https://artifacthub.devstats.cncf.io/',
    display_name: 'Artifact Hub',
    id: '00000000-0001-0000-0000-000000000000',
    home_url: 'https://artifacthub.io',
    logo_url:
      'https://raw.githubusercontent.com/cncf/artwork/master/projects/artifacthub/icon/color/artifacthub-icon-color.svg',
    maturity_id: 2,
    name: 'artifact-hub',
    organization: {
      name: 'artifact-hub',
    },
    rating: 'a',
    repositories: [
      {
        kind: RepositoryKind.Primary,
        name: 'artifact-hub',
        url: 'https://github.com/artifacthub/hub',
      },
    ],
    score: {
      best_practices: 100,
      documentation: 75,
      global: 89,
      license: 80,
      legal: 75,
      score_kind: ScoreKind.Primary,
      security: 100,
    },
    updated_at: 1645138013,
  },
  currentQueryString: '',
  saveScrollPosition: mockSaveScrollPosition,
};

describe('Card', () => {
  afterEach(() => {
    jest.resetAllMocks();
  });

  it('creates snapshot', () => {
    const { asFragment } = render(
      <Router>
        <Card {...defaultProps} />
      </Router>
    );
    expect(asFragment()).toMatchSnapshot();
  });

  describe('Render', () => {
    it('renders component', () => {
      render(
        <Router>
          <Card {...defaultProps} />{' '}
        </Router>
      );

      const images = screen.getAllByAltText('Artifact Hub logo');
      expect(images).toHaveLength(2);
      expect(images[0]).toHaveProperty(
        'src',
        'https://raw.githubusercontent.com/cncf/artwork/master/projects/artifacthub/icon/color/artifacthub-icon-color.svg'
      );

      expect(screen.getByText('Artifact Hub')).toBeInTheDocument();
      expect(screen.getByTestId('maturity-badge')).toBeInTheDocument();
      expect(screen.getByText('Sandbox')).toBeInTheDocument();
      expect(screen.getByTestId('category-badge')).toBeInTheDocument();
      expect(screen.getByText('App definition')).toBeInTheDocument();

      const repoLink = screen.getByRole('link', { name: 'Repository link' });
      expect(repoLink).toHaveProperty('href', 'https://github.com/artifacthub/hub');

      const statsLink = screen.getByRole('link', { name: 'Dev stats link' });
      expect(statsLink).toHaveProperty('href', 'https://artifacthub.devstats.cncf.io/');

      const globalScores = screen.getAllByTestId('global-score');
      expect(globalScores).toHaveLength(2);
      expect(globalScores[1]).toHaveTextContent('89');
    });
  });
});
