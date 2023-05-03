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
      expect(screen.getAllByText('Analytics')).toHaveLength(2);
      expect(screen.getAllByText('Artifact Hub badge')).toHaveLength(2);
      expect(screen.getByText('Contributor License Agreement')).toBeInTheDocument();
      expect(screen.getByText('CLA')).toBeInTheDocument();
      expect(screen.getAllByText('Community meeting')).toHaveLength(2);
      expect(screen.getByText('Developer Certificate of Origin')).toBeInTheDocument();
      expect(screen.getByText('DCO')).toBeInTheDocument();
      expect(screen.getAllByText('GitHub discussions')).toHaveLength(2);
      expect(screen.getByText('OpenSSF best practices')).toBeInTheDocument();
      expect(screen.getByText('OpenSSF best practices badge')).toBeInTheDocument();
      expect(screen.getAllByText('Recent release')).toHaveLength(2);
      expect(screen.getAllByText('Slack presence')).toHaveLength(2);

      expect(screen.getAllByRole('button', { name: /as passed/ })).toHaveLength(21);
      expect(screen.getAllByRole('button', { name: /as not passed/ })).toHaveLength(21);
    });

    it('renders option Slack presence as passed', () => {
      render(<Block {...defaultProps} activePassingChecks={[ReportOption.SlackPresence]} />);

      expect(screen.getAllByRole('button', { name: 'Unselect Slack presence as passed' })).toHaveLength(2);
    });

    it('renders option Slack presence as not passed', () => {
      render(<Block {...defaultProps} activeNotPassingChecks={[ReportOption.SlackPresence]} />);

      expect(screen.getAllByRole('button', { name: 'Unselect Slack presence as not passed' })).toHaveLength(2);
    });

    it('calls onChange to click remove one option', async () => {
      render(<Block {...defaultProps} activePassingChecks={[ReportOption.SlackPresence]} />);

      const btn = screen.getAllByRole('button', { name: 'Unselect Slack presence as passed' })[0];
      expect(btn).toBeInTheDocument();

      await userEvent.click(btn);

      expect(mockOnChange).toHaveBeenCalledTimes(1);
      expect(mockOnChange).toHaveBeenCalledWith('passing_check', 'slack_presence', false);
    });

    it('calls markAllAsPassedPerCategory to select all options as passed', async () => {
      const { rerender } = render(<Block {...defaultProps} />);

      expect(screen.getAllByRole('button', { name: 'Select Artifact Hub badge as passed' })).toHaveLength(2);

      const btn = screen.getByRole('button', { name: 'Mark all checks in Best Practices category as passed' });
      expect(btn).toBeInTheDocument();

      await userEvent.click(btn);

      expect(mockMarkAllAsPassedPerCategory).toHaveBeenCalledTimes(1);
      expect(mockMarkAllAsPassedPerCategory).toHaveBeenCalledWith('best_practices');

      rerender(<Block {...defaultProps} activePassingChecks={[ReportOption.ArtifactHubBadge]} />);

      expect(await screen.findAllByRole('button', { name: 'Unselect Artifact Hub badge as passed' })).toHaveLength(2);
    });

    it('calls markAllAsNotPassedPerCategory to select all options as not passed', async () => {
      const { rerender } = render(<Block {...defaultProps} />);

      expect(screen.getAllByRole('button', { name: 'Select Artifact Hub badge as not passed' })).toHaveLength(2);

      const btn = screen.getByRole('button', { name: 'Mark all checks in Best Practices category as not passed' });
      expect(btn).toBeInTheDocument();

      await userEvent.click(btn);

      expect(mockMarkAllAsNotPassedPerCategory).toHaveBeenCalledTimes(1);
      expect(mockMarkAllAsNotPassedPerCategory).toHaveBeenCalledWith('best_practices');

      rerender(<Block {...defaultProps} activeNotPassingChecks={[ReportOption.ArtifactHubBadge]} />);

      expect(await screen.findAllByRole('button', { name: 'Unselect Artifact Hub badge as not passed' })).toHaveLength(
        2
      );
    });

    it('calls resetChecksPerCategory to unselect all marked options', async () => {
      const { rerender } = render(
        <Block
          {...defaultProps}
          activePassingChecks={[ReportOption.Analytics, ReportOption.CLA]}
          activeNotPassingChecks={[ReportOption.ArtifactHubBadge]}
        />
      );

      expect(screen.getAllByRole('button', { name: 'Unselect Artifact Hub badge as not passed' })).toHaveLength(2);
      expect(screen.getAllByRole('button', { name: 'Select Artifact Hub badge as passed' })).toHaveLength(2);

      const btn = screen.getByRole('button', { name: 'Reset checks in Best Practices category' });
      expect(btn).toBeInTheDocument();

      await userEvent.click(btn);

      expect(mockResetChecksPerCategory).toHaveBeenCalledTimes(1);
      expect(mockResetChecksPerCategory).toHaveBeenCalledWith('best_practices');

      rerender(<Block {...defaultProps} />);

      expect(await screen.findAllByRole('button', { name: 'Select Artifact Hub badge as passed' })).toHaveLength(2);
      expect(screen.getAllByRole('button', { name: 'Select Artifact Hub badge as not passed' })).toHaveLength(2);
    });
  });
});
