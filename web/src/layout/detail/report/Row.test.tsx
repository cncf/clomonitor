import { render, screen } from '@testing-library/react';

import { ScoreType } from '../../../types';
import Row from './Row';

const defaultProps = {
  repoName: 'repo',
  reportId: 'id',
  name: ScoreType.Documentation,
  label: 'label',
  icon: <>icon</>,
  data: {
    roadmap: false,
    code_of_conduct: true,
    adopters: false,
    changelog: true,
    maintainers: true,
    website: true,
    contributing: true,
    governance: false,
    readme: true,
  },
  score: 90,
  getAnchorLink: jest.fn(),
};

describe('Row', () => {
  afterEach(() => {
    jest.resetAllMocks();
  });

  it('creates snapshot', () => {
    const { asFragment } = render(<Row {...defaultProps} />);

    expect(asFragment()).toMatchSnapshot();
  });

  describe('Render', () => {
    it('renders component', () => {
      render(<Row {...defaultProps} />);

      expect(screen.getByText('label')).toBeInTheDocument();
      expect(screen.getByText('90%')).toBeInTheDocument();
      expect(screen.getByText('icon')).toBeInTheDocument();
      expect(screen.getByRole('progressbar')).toBeInTheDocument();
      expect(screen.getByRole('progressbar')).toHaveStyle('width: 90%');
      expect(screen.getAllByTestId('opt-name')).toHaveLength(9);
    });

    it('renders options in correct order', () => {
      render(<Row {...defaultProps} />);

      const opts = screen.getAllByTestId('opt-name');
      expect(opts).toHaveLength(9);
      expect(opts[0]).toHaveTextContent('Adopters');
      expect(opts[1]).toHaveTextContent('Changelog');
      expect(opts[2]).toHaveTextContent('Code of conduct');
      expect(opts[3]).toHaveTextContent('Contributing');
      expect(opts[4]).toHaveTextContent('Governance');
      expect(opts[5]).toHaveTextContent('Maintainers');
      expect(opts[6]).toHaveTextContent('Readme');
      expect(opts[7]).toHaveTextContent('Roadmap');
      expect(opts[8]).toHaveTextContent('Website');
    });

    it('renders options in correct order', () => {
      render(<Row {...defaultProps} data={{ approved: true, scanning: 'http://url.com', spdx_id: 'Apache-2.0' }} />);

      const opts = screen.getAllByTestId('opt-name');
      expect(opts).toHaveLength(3);
      expect(opts[0]).toHaveTextContent('Apache-2.0');
      expect(opts[1]).toHaveTextContent('Approved license');
      expect(opts[2]).toHaveTextContent('License scanning');
    });

    it('renders component with recommended templates', () => {
      render(
        <Row
          {...defaultProps}
          recommendedTemplates={[
            { name: 'template1.md', url: 'http://template1.com' },
            { name: 'template2.md', url: 'http://template2.com' },
          ]}
        />
      );

      expect(screen.getByTestId('recommended-templates')).toBeInTheDocument();
      expect(screen.getByTestId('recommended-templates')).toHaveTextContent(
        'CNCF recommended templates: template1.md and template2.md.'
      );
      const links = screen.getAllByRole('link');
      expect(links).toHaveLength(2);
      expect(links[0]).toHaveProperty('href', 'http://template1.com/');
      expect(links[1]).toHaveProperty('href', 'http://template2.com/');
    });
  });
});
