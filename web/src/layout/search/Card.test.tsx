import { render, screen } from '@testing-library/react';
import { CheckSet } from 'clo-ui/components/CheckSetBadge';
import { Foundation } from 'clo-ui/components/Foundation';
import { Maturity } from 'clo-ui/components/Maturity';
import { BrowserRouter as Router } from 'react-router-dom';
import { vi } from 'vitest';

import Card from './Card';

vi.mock('date-fns', async () => {
  const actual = await vi.importActual<typeof import('date-fns')>('date-fns');
  return {
    ...actual,
    formatDistanceToNowStrict: () => '4 years ago',
  };
});

const mockSaveScrollPosition = vi.fn();

const defaultProps = {
  project: {
    accepted_at: 1592870400,
    category: 'app definition',
    description:
      'Artifact Hub is a web-based application that enables finding, installing, and publishing packages and configurations for CNCF projects.',
    devstats_url: 'https://artifacthub.devstats.cncf.io/',
    display_name: 'Artifact Hub',
    id: '00000000-0001-0000-0000-000000000000',
    home_url: 'https://artifacthub.io',
    logo_url:
      'https://raw.githubusercontent.com/cncf/artwork/master/projects/artifacthub/icon/color/artifacthub-icon-color.svg',
    maturity: Maturity.sandbox,
    foundation: Foundation.cncf,
    name: 'artifact-hub',
    rating: 'a',
    repositories: [
      {
        check_sets: [CheckSet.Community, CheckSet.Code],
        name: 'artifact-hub',
        url: 'https://github.com/artifacthub/hub',
        website_url: 'https://artifacthub.io',
      },
    ],
    score: {
      best_practices: 100,
      documentation: 75,
      global: 89,
      license: 80,
      legal: 75,
      security: 100,
    },
    updated_at: 1645138013,
  },
  currentQueryString: '',
  saveScrollPosition: mockSaveScrollPosition,
};

describe('Card', () => {
  afterEach(() => {
    vi.resetAllMocks();
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
    it('renders component', async () => {
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
      expect(screen.getByTestId('foundation-badge')).toBeInTheDocument();
      expect(screen.getByText('CNCF')).toBeInTheDocument();

      const repoLink = await screen.findByRole('link', { name: 'Repository link' });
      expect(repoLink).toHaveProperty('href', 'https://github.com/artifacthub/hub');

      const websiteLink = screen.getByRole('link', { name: 'Website link' });
      expect(websiteLink).toHaveProperty('href', 'https://artifacthub.io/');

      const statsLink = screen.getByRole('link', { name: 'Dev stats link' });
      expect(statsLink).toHaveProperty('href', 'https://artifacthub.devstats.cncf.io/');

      const globalScores = screen.getAllByTestId('global-score');
      expect(globalScores.length).toBeGreaterThan(0);
      expect(globalScores.at(-1)).toHaveTextContent('89');
    });
  });
});
