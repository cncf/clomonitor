import { act, render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';

import { ReportOption } from '../../../types';
import OptionCell from './OptionCell';
jest.mock('react-markdown', () => () => <div>markdown</div>);
jest.mock('rehype-external-links', () => () => <></>);

const defaultProps = {
  label: ReportOption.Adopters,
  check: {
    passed: true,
  },
};

const user = userEvent.setup({ delay: null });

describe('OptionCell', () => {
  afterEach(() => {
    jest.resetAllMocks();
  });

  it('creates snapshot', () => {
    const { asFragment } = render(
      <table>
        <tbody>
          <OptionCell {...defaultProps} />
        </tbody>
      </table>
    );

    expect(asFragment()).toMatchSnapshot();
  });

  describe('Render', () => {
    it('renders option', () => {
      render(
        <table>
          <tbody>
            <OptionCell {...defaultProps} />
          </tbody>
        </table>
      );

      expect(screen.getByText('Adopters')).toBeInTheDocument();
      expect(screen.getByTestId('opt-name')).toBeInTheDocument();
      expect(
        screen.getByText('List of organizations using this project in production or at stages of testing')
      ).toBeInTheDocument();
    });

    it('renders special SPDX option', () => {
      render(
        <table>
          <tbody>
            <OptionCell
              label={ReportOption.SPDX}
              check={{
                passed: true,
                value: 'Apache-2.0',
              }}
            />
          </tbody>
        </table>
      );

      expect(screen.getByText('Apache-2.0')).toBeInTheDocument();
      expect(screen.getByText(/file contains the repository's license/)).toBeInTheDocument();
      expect(screen.getByText('LICENSE')).toBeInTheDocument();
    });

    it('renders option with url', () => {
      render(
        <table>
          <tbody>
            <OptionCell
              {...defaultProps}
              check={{
                url: 'https://github.com/project-akri/akri/blob/main/ADOPTERS.md',
                exempt: false,
                failed: false,
                passed: true,
              }}
            />
          </tbody>
        </table>
      );

      const link = screen.getByRole('link', { name: 'Checks reference documentation' });
      expect(link).toBeInTheDocument();
      expect(link).toHaveTextContent('Adopters');
      expect(link).toHaveProperty('target', '_blank');
      expect(link).toHaveProperty('href', 'https://github.com/project-akri/akri/blob/main/ADOPTERS.md');
      expect(link).toHaveProperty('rel', 'noopener noreferrer');
    });

    it('renders option with details', async () => {
      jest.useFakeTimers();

      render(
        <table>
          <tbody>
            <OptionCell
              label={ReportOption.DependencyUpdateTool}
              check={{
                exempt: false,
                failed: false,
                passed: false,
                details:
                  '### Determines if the project uses a dependency update tool\n\n**OpenSSF Scorecard score**: 0\n**Reason**: no update tool detected\n\n**Details**:\n\nWarn: dependabot config file not detected in source location.\n\t\t\tWe recommend setting this configuration in code so it can be easily verified by others.\nWarn: renovatebot config file not detected in source location.\n\t\t\tWe recommend setting this configuration in code so it can be easily verified by others.\n\n*Please see the [check docs](https://github.com/ossf/scorecard/blob/33f80c93dc79f860d874857c511c4d26d399609d/docs/checks.md#dependency-update-tool) for more details*',
              }}
            />
          </tbody>
        </table>
      );

      const icons = screen.getAllByTestId('error-icon');
      expect(icons).toHaveLength(2);

      const dropdown = screen.getByRole('complementary');

      expect(dropdown).not.toHaveClass('show');

      await user.hover(icons[0]);

      act(() => {
        jest.advanceTimersByTime(100);
      });

      expect(dropdown).toHaveClass('show');
      expect(screen.getByText('markdown')).toBeInTheDocument();

      jest.useRealTimers();
    });

    describe('passed', () => {
      it('when true: renders option with success icon', () => {
        render(
          <table>
            <tbody>
              <OptionCell {...defaultProps} />
            </tbody>
          </table>
        );

        expect(screen.getByTestId('success-icon')).toBeInTheDocument();
        expect(screen.queryByTestId('error-icon')).toBeNull();
      });

      it('when false: renders option with error icon', () => {
        render(
          <table>
            <tbody>
              <OptionCell
                {...defaultProps}
                check={{
                  passed: false,
                }}
              />
            </tbody>
          </table>
        );

        expect(screen.getByTestId('error-icon')).toBeInTheDocument();
        expect(screen.queryByTestId('success-icon')).toBeNull();
      });
    });

    describe('exempt', () => {
      it('when true', () => {
        render(
          <table>
            <tbody>
              <OptionCell
                label={ReportOption.Adopters}
                check={{
                  passed: false,
                  exempt: true,
                  exemption_reason: 'this is a sample reason',
                }}
              />
            </tbody>
          </table>
        );

        expect(screen.getAllByTestId('exempt-icon')).toHaveLength(2);
        expect(screen.queryByTestId('failed-icon')).toBeNull();
        expect(screen.queryByTestId('success-icon')).toBeNull();
        expect(screen.queryByTestId('error-icon')).toBeNull();
      });

      it('displays reason tooltip', async () => {
        jest.useFakeTimers();

        render(
          <table>
            <tbody>
              <OptionCell
                label={ReportOption.Adopters}
                check={{
                  passed: false,
                  exempt: true,
                  exemption_reason: 'this is a sample reason',
                }}
              />
            </tbody>
          </table>
        );

        const icon = screen.getByTestId('elementWithTooltip');
        await user.hover(icon);

        expect(await screen.findByRole('tooltip')).toBeInTheDocument();
        expect(screen.getByText('This repository is exempt from passing this check')).toBeInTheDocument();
        expect(screen.getByText('Reason:')).toBeInTheDocument();
        expect(screen.getByText(/this is a sample reason/)).toBeInTheDocument();

        jest.useRealTimers();
      });
    });

    describe('failed', () => {
      it('when true', () => {
        render(
          <table>
            <tbody>
              <OptionCell
                label={ReportOption.Adopters}
                check={{
                  passed: false,
                  failed: true,
                  fail_reason: 'this is a sample reason',
                }}
              />
            </tbody>
          </table>
        );

        expect(screen.getAllByTestId('failed-icon')).toHaveLength(2);
        expect(screen.queryByTestId('exempt-icon')).toBeNull();
        expect(screen.queryByTestId('success-icon')).toBeNull();
        expect(screen.queryByTestId('error-icon')).toBeNull();
      });

      it('displays reason tooltip', async () => {
        jest.useFakeTimers();

        render(
          <table>
            <tbody>
              <OptionCell
                label={ReportOption.Adopters}
                check={{
                  passed: false,
                  failed: true,
                  fail_reason: 'this is a sample reason',
                }}
              />
            </tbody>
          </table>
        );

        const icon = screen.getByTestId('elementWithTooltip');
        await user.hover(icon);

        expect(await screen.findByRole('tooltip')).toBeInTheDocument();
        expect(screen.getByText('Something went wrong running this check')).toBeInTheDocument();
        expect(screen.getByText('Reason:')).toBeInTheDocument();
        expect(screen.getByText(/this is a sample reason/)).toBeInTheDocument();

        jest.useRealTimers();
      });
    });
  });
});
