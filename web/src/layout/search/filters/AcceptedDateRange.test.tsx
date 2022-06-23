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
      expect(screen.getByText('Jan 1, 2016')).toBeInTheDocument();
      expect(screen.getByText('Mar 24, 2022')).toBeInTheDocument();
      expect(screen.getByRole('button', { name: 'Open calendar to choose date accepted_from' })).toBeInTheDocument();
      expect(screen.getByRole('button', { name: 'Open calendar to choose date accepted_to' })).toBeInTheDocument();

      const dropdowns = screen.getAllByRole('complementary');
      expect(dropdowns).toHaveLength(2);
      expect(dropdowns[0]).not.toHaveClass('show');
      expect(dropdowns[1]).not.toHaveClass('show');
    });

    it('displays calendar', async () => {
      render(<AcceptedDateRange {...defaultProps} />);

      const btn = screen.getByRole('button', { name: 'Open calendar to choose date accepted_from' });
      await userEvent.click(btn);

      expect(screen.getAllByRole('complementary')[0]).toHaveClass('show');
    });

    it('calls mockOnAcceptedDateRangeChange', async () => {
      render(<AcceptedDateRange {...defaultProps} />);

      const btn = screen.getByRole('button', { name: 'Open calendar to choose date accepted_from' });
      await userEvent.click(btn);

      expect(screen.getAllByRole('complementary')[0]).toHaveClass('show');

      const day20Btn = screen.getAllByText('20')[0].closest('button');
      await userEvent.click(day20Btn as HTMLButtonElement);

      expect(mockOnAcceptedDateRangeChange).toHaveBeenCalledTimes(1);
      expect(mockOnAcceptedDateRangeChange).toHaveBeenCalledWith({
        accepted_from: '2016-01-20',
      });
    });

    it('does not call to mockOnAcceptedDateRangeChange when range is the same than received props', async () => {
      render(<AcceptedDateRange {...defaultProps} acceptedFrom="2018-01-02" />);

      expect(mockOnAcceptedDateRangeChange).toHaveBeenCalledTimes(0);
    });
  });
});
