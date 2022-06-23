import { render, screen } from '@testing-library/react';

import ProgressBar from './ProgressBar';

const defaultProps = {
  title: 'Security',
  icon: <>icon</>,
  value: 75,
};

describe('ProgressBar', () => {
  afterEach(() => {
    jest.resetAllMocks();
  });

  it('creates snapshot', () => {
    const { asFragment } = render(<ProgressBar {...defaultProps} />);

    expect(asFragment()).toMatchSnapshot();
  });

  describe('Render', () => {
    it('renders bar', () => {
      render(<ProgressBar {...defaultProps} />);

      expect(screen.getByText('Security')).toBeInTheDocument();
      expect(screen.getByText('icon')).toBeInTheDocument();
      expect(screen.getByText('75%')).toBeInTheDocument();

      const progressBar = screen.getByRole('progressbar');
      expect(progressBar).toBeInTheDocument();
      expect(progressBar).toHaveStyle({ backgroundColor: 'var(--rm-green)' });
      expect(progressBar).toHaveStyle({ width: '75%' });
    });

    it('renders bar - 20', () => {
      render(<ProgressBar {...defaultProps} value={20} />);

      expect(screen.getByText('20%')).toBeInTheDocument();

      const progressBar = screen.getByRole('progressbar');
      expect(progressBar).toBeInTheDocument();
      expect(progressBar).toHaveStyle({ backgroundColor: 'var(--rm-red)' });
      expect(progressBar).toHaveStyle({ width: '20%' });
    });

    it('renders bar - 40', () => {
      render(<ProgressBar {...defaultProps} value={40} />);

      expect(screen.getByText('40%')).toBeInTheDocument();

      const progressBar = screen.getByRole('progressbar');
      expect(progressBar).toBeInTheDocument();
      expect(progressBar).toHaveStyle({ backgroundColor: 'var(--rm-orange)' });
      expect(progressBar).toHaveStyle({ width: '40%' });
    });

    it('renders bar - 60', () => {
      render(<ProgressBar {...defaultProps} value={60} />);

      expect(screen.getByText('60%')).toBeInTheDocument();

      const progressBar = screen.getByRole('progressbar');
      expect(progressBar).toBeInTheDocument();
      expect(progressBar).toHaveStyle({ backgroundColor: 'var(--rm-yellow)' });
      expect(progressBar).toHaveStyle({ width: '60%' });
    });

    it('renders bar - 80', () => {
      render(<ProgressBar {...defaultProps} value={80} />);

      expect(screen.getByText('80%')).toBeInTheDocument();

      const progressBar = screen.getByRole('progressbar');
      expect(progressBar).toBeInTheDocument();
      expect(progressBar).toHaveStyle({ backgroundColor: 'var(--rm-green)' });
      expect(progressBar).toHaveStyle({ width: '80%' });
    });
  });
});
