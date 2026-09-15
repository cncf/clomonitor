import { render, screen } from '@testing-library/react';
import { vi } from 'vitest';

import Checks from './Checks';

const defaultProps = {
  data: {
    analytics: 8,
    artifacthub_badge: 9,
    cla: 12,
    community_meeting: 43,
    dco: 83,
    openssf_badge: 59,
    recent_release: 81,
    slack_presence: 33,
  },
  title: 'Best Practices',
  onSelectCheck: vi.fn(),
};

describe('Checks', () => {
  afterEach(() => {
    vi.resetAllMocks();
  });

  it('creates snapshot', () => {
    const { asFragment } = render(<Checks {...defaultProps} />);

    expect(asFragment()).toMatchSnapshot();
  });

  describe('Render', () => {
    it('renders component', () => {
      render(<Checks {...defaultProps} />);

      expect(screen.getByText('Best Practices')).toBeInTheDocument();
      expect(screen.getAllByText('Analytics')).toHaveLength(2);
      expect(screen.getByText('8%')).toBeInTheDocument();
      expect(screen.getAllByText('Artifact Hub badge')).toHaveLength(2);
      expect(screen.getByText('9%')).toBeInTheDocument();
      expect(screen.getAllByText('CLA')).toHaveLength(2);
      expect(screen.getByText('12%')).toBeInTheDocument();
      expect(screen.getAllByText('Community meeting')).toHaveLength(2);
      expect(screen.getByText('43%')).toBeInTheDocument();
      expect(screen.getAllByText('DCO')).toHaveLength(2);
      expect(screen.getByText('83%')).toBeInTheDocument();
      expect(screen.getAllByText('OpenSSF best practices')).toHaveLength(2);
      expect(screen.getByText('59%')).toBeInTheDocument();
      expect(screen.getAllByText('Recent release')).toHaveLength(2);
      expect(screen.getByText('81%')).toBeInTheDocument();
      expect(screen.getAllByText('Slack presence')).toHaveLength(2);
      expect(screen.getByText('33%')).toBeInTheDocument();

      expect(screen.getAllByRole('progressbar')).toHaveLength(8);
    });

    it('renders checks properly sorted', () => {
      render(
        <Checks
          data={{ license_approved: 93, license_scanning: 22, license_spdx_id: 94 }}
          title="License"
          onSelectCheck={vi.fn()}
        />
      );

      expect(screen.getAllByText('License found')).toHaveLength(2);
      expect(screen.getAllByText('Approved license')).toHaveLength(2);
      expect(screen.getAllByText('License scanning')).toHaveLength(2);

      const progressbar = screen.getAllByRole('progressbar');
      expect(progressbar[0]).toHaveStyle({ width: '94%' });
      expect(progressbar[1]).toHaveStyle({ width: '93%' });
      expect(progressbar[2]).toHaveStyle({ width: '22%' });
    });

    it('renders Agent Readiness checks with compact names', () => {
      render(
        <Checks
          data={{
            authentication: 52,
            content_discoverability: 71,
            content_structure: 48,
            markdown_availability: 66,
            observability: 41,
            page_size: 38,
            url_stability: 64,
          }}
          title="Agent Readiness"
          onSelectCheck={vi.fn()}
        />
      );

      expect(screen.getByText('Agent Readiness')).toBeInTheDocument();
      expect(screen.getAllByText('Authentication')).toHaveLength(2);
      expect(screen.getAllByText('Content structure')).toHaveLength(2);
      expect(screen.getAllByText('Discoverability')).toHaveLength(2);
      expect(screen.getAllByText('Markdown')).toHaveLength(2);
      expect(screen.getAllByText('Observability')).toHaveLength(2);
      expect(screen.getAllByText('Page size')).toHaveLength(2);
      expect(screen.getAllByText('URL stability')).toHaveLength(2);
      expect(screen.getAllByRole('progressbar')).toHaveLength(7);
    });
  });
});
