import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';

import BadgeCell from './BadgeCell';

const mockOnClick = jest.fn();

const defaultProps = {
  onClick: mockOnClick,
};

describe('Badge', () => {
  afterEach(() => {
    jest.resetAllMocks();
  });

  it('creates snapshot', () => {
    const { asFragment } = render(
      <table>
        <tbody>
          <tr>
            <BadgeCell {...defaultProps} value={80} />
          </tr>
        </tbody>
      </table>
    );

    expect(asFragment()).toMatchSnapshot();
  });

  describe('Render', () => {
    it('renders badge', () => {
      render(
        <table>
          <tbody>
            <tr>
              <BadgeCell {...defaultProps} value={80} />
            </tr>
          </tbody>
        </table>
      );

      expect(screen.getByText('80')).toBeInTheDocument();
    });

    it('renders badge with undefined value', () => {
      render(
        <table>
          <tbody>
            <tr>
              <BadgeCell {...defaultProps} />
            </tr>
          </tbody>
        </table>
      );

      expect(screen.getByText('n/a')).toBeInTheDocument();
      expect(screen.queryByRole('button')).toBeNull();
    });

    it('clicks on anchor', () => {
      render(
        <table>
          <tbody>
            <tr>
              <BadgeCell {...defaultProps} value={70} />
            </tr>
          </tbody>
        </table>
      );

      const btn = screen.getByRole('button');
      userEvent.click(btn);

      expect(mockOnClick).toHaveBeenCalledTimes(1);
    });
  });
});
