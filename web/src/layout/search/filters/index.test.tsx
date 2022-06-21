import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';

import Filters from './index';

const mockOnChange = jest.fn();
const mockOnAcceptedDateRangeChange = jest.fn();

const defaultProps = {
  visibleTitle: true,
  activeFilters: {},
  onChange: mockOnChange,
  onAcceptedDateRangeChange: mockOnAcceptedDateRangeChange,
  device: 'test',
};

describe('Filters', () => {
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
    const { asFragment } = render(<Filters {...defaultProps} />);

    expect(asFragment()).toMatchSnapshot();
  });

  describe('Render', () => {
    it('renders Filters', () => {
      render(<Filters {...defaultProps} />);

      expect(screen.getByText('Filters')).toBeInTheDocument();

      expect(screen.getByText('Foundation')).toBeInTheDocument();
      expect(screen.getByRole('checkbox', { name: 'CNCF' })).toBeInTheDocument();
      expect(screen.getByRole('checkbox', { name: 'LF AI & Data' })).toBeInTheDocument();

      expect(screen.getByText('Maturity level')).toBeInTheDocument();
      expect(screen.getByRole('checkbox', { name: 'Graduated' })).toBeInTheDocument();
      expect(screen.getByRole('checkbox', { name: 'Incubating' })).toBeInTheDocument();
      expect(screen.getByRole('checkbox', { name: 'Sandbox' })).toBeInTheDocument();

      expect(screen.getByText('Rating')).toBeInTheDocument();
      expect(screen.getByRole('checkbox', { name: 'A [75-100]' })).toBeInTheDocument();
      expect(screen.getByRole('checkbox', { name: 'B [50-74]' })).toBeInTheDocument();
      expect(screen.getByRole('checkbox', { name: 'C [25-49]' })).toBeInTheDocument();
      expect(screen.getByRole('checkbox', { name: 'D [0-24]' })).toBeInTheDocument();

      expect(screen.getAllByText('From:')).toHaveLength(2);
      expect(screen.getAllByText('To:')).toHaveLength(2);
      expect(screen.getByText('Jan 1, 2016 - Mar 24, 2022')).toBeInTheDocument();
    });

    it('renders Filters with selected options', () => {
      render(<Filters {...defaultProps} activeFilters={{ maturity: ['sandbox'], rating: ['a', 'b'] }} />);

      expect(screen.getByRole('checkbox', { name: 'Sandbox' })).toBeChecked();
      expect(screen.getByRole('checkbox', { name: 'A [75-100]' })).toBeChecked();
      expect(screen.getByRole('checkbox', { name: 'B [50-74]' })).toBeChecked();
    });

    it('calls onChange to click filter', async () => {
      render(<Filters {...defaultProps} />);

      const check = screen.getByRole('checkbox', { name: 'Sandbox' });

      expect(check).not.toBeChecked();

      await userEvent.click(check);

      expect(mockOnChange).toHaveBeenCalledTimes(1);
      expect(mockOnChange).toHaveBeenCalledWith('maturity', 'sandbox', true);
    });
  });
});
