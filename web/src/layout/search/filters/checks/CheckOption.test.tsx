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

      expect(screen.getAllByText('Artifact Hub badge')).toHaveLength(2);

      expect(screen.getAllByRole('button', { name: 'Select Artifact Hub badge as passed' })).toHaveLength(2);
      expect(screen.getAllByRole('button', { name: 'Select Artifact Hub badge as not passed' })).toHaveLength(2);
    });

    it('renders option Artifact Hub badge as passed', () => {
      render(<CheckOption {...defaultProps} activePassingChecks={[ReportOption.ArtifactHubBadge]} />);

      expect(screen.getAllByRole('button', { name: 'Unselect Artifact Hub badge as passed' })).toHaveLength(2);
      expect(screen.getAllByRole('button', { name: 'Select Artifact Hub badge as not passed' })).toHaveLength(2);
      expect(screen.getAllByRole('button', { name: 'Unselect Artifact Hub badge as passed' })[0]).toHaveClass(
        'isPassing'
      );
    });

    it('renders option Artifact Hub badge as not passed', () => {
      render(<CheckOption {...defaultProps} activeNotPassingChecks={[ReportOption.ArtifactHubBadge]} />);

      expect(screen.getAllByRole('button', { name: 'Select Artifact Hub badge as passed' })).toHaveLength(2);
      expect(screen.getAllByRole('button', { name: 'Unselect Artifact Hub badge as not passed' })).toHaveLength(2);
      expect(screen.getAllByRole('button', { name: 'Unselect Artifact Hub badge as not passed' })[0]).toHaveClass(
        'isNotPassing'
      );
    });

    it('calls onChange to click an unchecked option', async () => {
      render(<CheckOption {...defaultProps} activePassingChecks={[ReportOption.ArtifactHubBadge]} />);

      const btn = screen.getAllByRole('button', { name: 'Unselect Artifact Hub badge as passed' })[0];
      expect(btn).toBeInTheDocument();

      await userEvent.click(btn);

      expect(mockOnChange).toHaveBeenCalledTimes(1);
      expect(mockOnChange).toHaveBeenCalledWith('passing_check', 'artifacthub_badge', false);
    });
  });
});
