import { render, screen } from '@testing-library/react';

import Checks from './Checks';

const defaultProps = {
  data: {
    artifacthub_badge: 9,
    cla: 12,
    community_meeting: 43,
    dco: 83,
    ga4: 8,
    openssf_badge: 59,
    recent_release: 81,
    slack_presence: 33,
  },
  title: 'Best Practices',
};

describe('Checks', () => {
  afterEach(() => {
    jest.resetAllMocks();
  });

  it('creates snapshot', () => {
    const { asFragment } = render(<Checks {...defaultProps} />);

    expect(asFragment()).toMatchSnapshot();
  });

  describe('Render', () => {
    it('renders component', () => {
      render(<Checks {...defaultProps} />);

      expect(screen.getByText('Best Practices')).toBeInTheDocument();
      expect(screen.getByText('Artifact Hub badge')).toBeInTheDocument();
      expect(screen.getByText('9%')).toBeInTheDocument();
      expect(screen.getByText('CLA')).toBeInTheDocument();
      expect(screen.getByText('12%')).toBeInTheDocument();
      expect(screen.getByText('Community meeting')).toBeInTheDocument();
      expect(screen.getByText('43%')).toBeInTheDocument();
      expect(screen.getByText('DCO')).toBeInTheDocument();
      expect(screen.getByText('83%')).toBeInTheDocument();
      expect(screen.getByText('Google Analytics 4')).toBeInTheDocument();
      expect(screen.getByText('8%')).toBeInTheDocument();
      expect(screen.getByText('OpenSSF badge')).toBeInTheDocument();
      expect(screen.getByText('59%')).toBeInTheDocument();
      expect(screen.getByText('Recent release')).toBeInTheDocument();
      expect(screen.getByText('81%')).toBeInTheDocument();
      expect(screen.getByText('Slack presence')).toBeInTheDocument();
      expect(screen.getByText('33%')).toBeInTheDocument();

      expect(screen.getAllByRole('progressbar')).toHaveLength(8);
    });

    it('renders checks properly sorted', () => {
      render(<Checks data={{ license_approved: 93, license_scanning: 22, license_spdx_id: 94 }} title="License" />);

      expect(screen.getAllByText('License')).toHaveLength(2);
      expect(screen.getByText('Approved license')).toBeInTheDocument();
      expect(screen.getByText('License scanning')).toBeInTheDocument();

      const progressbar = screen.getAllByRole('progressbar');
      expect(progressbar[0]).toHaveStyle({ width: '94%' });
      expect(progressbar[1]).toHaveStyle({ width: '93%' });
      expect(progressbar[2]).toHaveStyle({ width: '22%' });
    });
  });
});
