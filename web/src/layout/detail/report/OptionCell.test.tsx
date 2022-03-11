import { render, screen } from '@testing-library/react';

import { ReportOption } from '../../../types';
import OptionCell from './OptionCell';

const defaultProps = {
  label: ReportOption.Adopters,
  check: {
    passed: true,
  },
};

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
  });
});
