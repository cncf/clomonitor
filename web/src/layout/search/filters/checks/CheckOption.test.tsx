import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';

import { ReportOption, ScoreType } from '../../../../types';
import CheckOption from './CheckOption';

const mockOnChange = jest.fn();

const defaultProps = {
  type: ScoreType.BestPractices,
  option: ReportOption.ArtifactHubBadge,
  activePassingChecks: [],
  activeNotPassingChecks: [],
  onChange: mockOnChange,
};

describe('CheckOption', () => {
  afterEach(() => {
    jest.resetAllMocks();
  });

  it('creates snapshot', () => {
    const { asFragment } = render(<CheckOption {...defaultProps} />);

    expect(asFragment()).toMatchSnapshot();
  });

  describe('Render', () => {
    it('renders Checks', () => {
      render(<CheckOption {...defaultProps} />);

      expect(screen.getByText('Artifact Hub badge')).toBeInTheDocument();

      expect(screen.getByRole('button', { name: 'Select Artifact Hub badge as passed' })).toBeInTheDocument();
      expect(screen.getByRole('button', { name: 'Select Artifact Hub badge as not passed' })).toBeInTheDocument();
    });

    it('renders option Artifact Hub badge as passed', () => {
      render(<CheckOption {...defaultProps} activePassingChecks={[ReportOption.ArtifactHubBadge]} />);

      expect(screen.getByRole('button', { name: 'Unselect Artifact Hub badge as passed' })).toBeInTheDocument();
      expect(screen.getByRole('button', { name: 'Select Artifact Hub badge as not passed' })).toBeInTheDocument();
    });

    it('renders option Artifact Hub badge as not passed', () => {
      render(<CheckOption {...defaultProps} activeNotPassingChecks={[ReportOption.ArtifactHubBadge]} />);

      expect(screen.getByRole('button', { name: 'Select Artifact Hub badge as passed' })).toBeInTheDocument();
      expect(screen.getByRole('button', { name: 'Unselect Artifact Hub badge as not passed' })).toBeInTheDocument();
    });

    it('calls onChange to click an unchecked option', async () => {
      render(<CheckOption {...defaultProps} activePassingChecks={[ReportOption.ArtifactHubBadge]} />);

      const btn = screen.getByRole('button', { name: 'Unselect Artifact Hub badge as passed' });
      expect(btn).toBeInTheDocument();

      await userEvent.click(btn);

      expect(mockOnChange).toHaveBeenCalledTimes(1);
      expect(mockOnChange).toHaveBeenCalledWith('passing_check', 'artifacthub_badge', false);
    });
  });
});
