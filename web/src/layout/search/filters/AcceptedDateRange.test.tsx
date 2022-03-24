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

      expect(screen.getByText('Accepted')).toBeInTheDocument();
      expect(screen.getByText('2016')).toBeInTheDocument();
      expect(screen.getByText("'17")).toBeInTheDocument();
      expect(screen.getByText("'18")).toBeInTheDocument();
      expect(screen.getByText("'19")).toBeInTheDocument();
      expect(screen.getByText("'20")).toBeInTheDocument();
      expect(screen.getByText("'21")).toBeInTheDocument();
      expect(screen.getByText('2022')).toBeInTheDocument();
      expect(screen.getAllByRole('slider')).toHaveLength(2);
    });

    it('calls mockOnAcceptedDateRangeChange', () => {
      render(<AcceptedDateRange {...defaultProps} />);

      const mark = screen.getByText("'20");
      userEvent.click(mark);

      expect(mockOnAcceptedDateRangeChange).toHaveBeenCalledTimes(1);
      expect(mockOnAcceptedDateRangeChange).toHaveBeenCalledWith({
        accepted_from: undefined,
        accepted_to: '2020-12-31',
      });
    });

    it('calls mockOnAcceptedDateRangeChange twice', () => {
      render(<AcceptedDateRange {...defaultProps} />);

      const mark20 = screen.getByText("'20");
      userEvent.click(mark20);

      const mark18 = screen.getByText("'18");
      userEvent.click(mark18);

      expect(mockOnAcceptedDateRangeChange).toHaveBeenCalledTimes(2);
      expect(mockOnAcceptedDateRangeChange).toHaveBeenCalledWith({
        accepted_from: undefined,
        accepted_to: '2020-12-31',
      });
      expect(mockOnAcceptedDateRangeChange).toHaveBeenLastCalledWith({
        accepted_from: undefined,
        accepted_to: '2018-12-31',
      });
    });
  });
});
