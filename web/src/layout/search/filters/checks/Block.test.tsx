import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';

import { ReportOption, ScoreType } from '../../../../types';
import Block from './Block';

const mockOnChange = jest.fn();
const mockResetChecksPerCategory = jest.fn();
const mockMarkAllAsPassedPerCategory = jest.fn();
const mockMarkAllAsNotPassedPerCategory = jest.fn();

const defaultProps = {
  type: ScoreType.BestPractices,
  activePassingChecks: [],
  activeNotPassingChecks: [],
  onChange: mockOnChange,
  resetChecksPerCategory: mockResetChecksPerCategory,
  markAllAsPassedPerCategory: mockMarkAllAsPassedPerCategory,
  markAllAsNotPassedPerCategory: mockMarkAllAsNotPassedPerCategory,
};

describe('Block', () => {
  afterEach(() => {
    jest.resetAllMocks();
  });

  it('creates snapshot', () => {
    const { asFragment } = render(<Block {...defaultProps} />);

    expect(asFragment()).toMatchSnapshot();
  });

  describe('Render', () => {
    it('renders Checks', () => {
      render(<Block {...defaultProps} />);

      expect(screen.getByText('Best Practices')).toBeInTheDocument();
      expect(
        screen.getByRole('button', { name: 'Mark all checks in Best Practices category as passed' })
      ).toBeInTheDocument();
      expect(
        screen.getByRole('button', { name: 'Mark all checks in Best Practices category as not passed' })
      ).toBeInTheDocument();
      expect(screen.getByRole('button', { name: 'Reset checks in Best Practices category' })).toBeInTheDocument();
      expect(screen.getByText('Artifact Hub badge')).toBeInTheDocument();
      expect(screen.getByText('Contributor License Agreement')).toBeInTheDocument();
      expect(screen.getByText('Community meeting')).toBeInTheDocument();
      expect(screen.getByText('Developer Certificate of Origin')).toBeInTheDocument();
      expect(screen.getByText('Google Analytics 4')).toBeInTheDocument();
      expect(screen.getByText('GitHub discussions')).toBeInTheDocument();
      expect(screen.getByText('OpenSSF badge')).toBeInTheDocument();
      expect(screen.getByText('Recent release')).toBeInTheDocument();
      expect(screen.getByText('Slack presence')).toBeInTheDocument();

      expect(screen.getAllByRole('button', { name: /as passed/ })).toHaveLength(10);
      expect(screen.getAllByRole('button', { name: /as not passed/ })).toHaveLength(10);
    });

    it('renders option Slack presence as passed', () => {
      render(<Block {...defaultProps} activePassingChecks={[ReportOption.SlackPresence]} />);

      expect(screen.getByRole('button', { name: 'Unselect Slack presence as passed' })).toBeInTheDocument();
    });

    it('renders option Slack presence as not passed', () => {
      render(<Block {...defaultProps} activeNotPassingChecks={[ReportOption.SlackPresence]} />);

      expect(screen.getByRole('button', { name: 'Unselect Slack presence as not passed' })).toBeInTheDocument();
    });

    it('calls onChange to click remove one option', async () => {
      render(<Block {...defaultProps} activePassingChecks={[ReportOption.SlackPresence]} />);

      const btn = screen.getByRole('button', { name: 'Unselect Slack presence as passed' });
      expect(btn).toBeInTheDocument();

      await userEvent.click(btn);

      expect(mockOnChange).toHaveBeenCalledTimes(1);
      expect(mockOnChange).toHaveBeenCalledWith('passing_check', 'slack_presence', false);
    });

    it('calls markAllAsPassedPerCategory to select all options as passed', async () => {
      const { rerender } = render(<Block {...defaultProps} />);

      expect(screen.getByRole('button', { name: 'Select Artifact Hub badge as passed' })).toBeInTheDocument();

      const btn = screen.getByRole('button', { name: 'Mark all checks in Best Practices category as passed' });
      expect(btn).toBeInTheDocument();

      await userEvent.click(btn);

      expect(mockMarkAllAsPassedPerCategory).toHaveBeenCalledTimes(1);
      expect(mockMarkAllAsPassedPerCategory).toHaveBeenCalledWith('best_practices');

      rerender(<Block {...defaultProps} activePassingChecks={[ReportOption.ArtifactHubBadge]} />);

      expect(await screen.findByRole('button', { name: 'Unselect Artifact Hub badge as passed' })).toBeInTheDocument();
    });

    it('calls markAllAsNotPassedPerCategory to select all options as not passed', async () => {
      const { rerender } = render(<Block {...defaultProps} />);

      expect(screen.getByRole('button', { name: 'Select Artifact Hub badge as not passed' })).toBeInTheDocument();

      const btn = screen.getByRole('button', { name: 'Mark all checks in Best Practices category as not passed' });
      expect(btn).toBeInTheDocument();

      await userEvent.click(btn);

      expect(mockMarkAllAsNotPassedPerCategory).toHaveBeenCalledTimes(1);
      expect(mockMarkAllAsNotPassedPerCategory).toHaveBeenCalledWith('best_practices');

      rerender(<Block {...defaultProps} activeNotPassingChecks={[ReportOption.ArtifactHubBadge]} />);

      expect(
        await screen.findByRole('button', { name: 'Unselect Artifact Hub badge as not passed' })
      ).toBeInTheDocument();
    });

    it('calls resetChecksPerCategory to unselect all marked options', async () => {
      const { rerender } = render(
        <Block
          {...defaultProps}
          activePassingChecks={[ReportOption.CLA, ReportOption.GA4]}
          activeNotPassingChecks={[ReportOption.ArtifactHubBadge]}
        />
      );

      expect(screen.getByRole('button', { name: 'Unselect Artifact Hub badge as not passed' })).toBeInTheDocument();
      expect(screen.getByRole('button', { name: 'Select Artifact Hub badge as passed' })).toBeInTheDocument();

      const btn = screen.getByRole('button', { name: 'Reset checks in Best Practices category' });
      expect(btn).toBeInTheDocument();

      await userEvent.click(btn);

      expect(mockResetChecksPerCategory).toHaveBeenCalledTimes(1);
      expect(mockResetChecksPerCategory).toHaveBeenCalledWith('best_practices');

      rerender(<Block {...defaultProps} />);

      expect(await screen.findByRole('button', { name: 'Select Artifact Hub badge as passed' })).toBeInTheDocument();
      expect(screen.getByRole('button', { name: 'Select Artifact Hub badge as not passed' })).toBeInTheDocument();
    });
  });
});
