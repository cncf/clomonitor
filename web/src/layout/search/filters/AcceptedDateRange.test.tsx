import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';

import AcceptedDateRange from './AcceptedDateRange';

const mockOnAcceptedDateRangeChange = jest.fn();

const defaultProps = {
  onAcceptedDateRangeChange: mockOnAcceptedDateRangeChange,
};

describe('AcceptedDateRange', () => {
  let dateNowSpy: any;

  beforeEach(() => {
    dateNowSpy = jest.spyOn(Date, 'now').mockImplementation(() => 1648154630000);
  });

  afterAll(() => {
    dateNowSpy.mockRestore();
  });

  afterEach(() => {
    jest.resetAllMocks();
  });

  it('creates snapshot', () => {
    const { asFragment } = render(<AcceptedDateRange {...defaultProps} />);

    expect(asFragment()).toMatchSnapshot();
  });

  describe('Render', () => {
    it('renders', () => {
      render(<AcceptedDateRange {...defaultProps} />);

      expect(screen.getAllByText('From:')).toHaveLength(2);
      expect(screen.getAllByText('To:')).toHaveLength(2);
      expect(screen.getByText('Jan 1, 2016 - Mar 24, 2022')).toBeInTheDocument();
      expect(screen.getByRole('button', { name: 'Open calendar' })).toBeInTheDocument();
      expect(screen.getByRole('complementary')).not.toHaveClass('show');
    });

    it('displays calendar', async () => {
      render(<AcceptedDateRange {...defaultProps} />);

      const btn = screen.getByRole('button', { name: 'Open calendar' });
      await userEvent.click(btn);

      expect(screen.getByRole('complementary')).toHaveClass('show');
    });

    it('calls mockOnAcceptedDateRangeChange', async () => {
      render(<AcceptedDateRange {...defaultProps} />);

      const btn = screen.getByRole('button', { name: 'Open calendar' });
      await userEvent.click(btn);

      expect(screen.getByRole('complementary')).toHaveClass('show');

      const day20Btn = screen.getByText('20').closest('button');
      await userEvent.click(day20Btn as HTMLButtonElement);

      expect(mockOnAcceptedDateRangeChange).toHaveBeenCalledTimes(1);
      expect(mockOnAcceptedDateRangeChange).toHaveBeenCalledWith({
        accepted_from: '2016-01-20',
        accepted_to: '2016-01-20',
      });
    });
  });
});
