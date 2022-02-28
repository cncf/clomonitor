import { render, screen } from '@testing-library/react';

import { ScoreType } from '../../../types';
import Row from './Row';

const defaultProps = {
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
  });
});
