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

      expect(screen.getByText('Maturity level')).toBeInTheDocument();
      expect(screen.getByRole('checkbox', { name: 'Graduated' })).toBeInTheDocument();
      expect(screen.getByRole('checkbox', { name: 'Incubating' })).toBeInTheDocument();
      expect(screen.getByRole('checkbox', { name: 'Sandbox' })).toBeInTheDocument();

      expect(screen.getByText('Rating')).toBeInTheDocument();
      expect(screen.getByRole('checkbox', { name: 'A [75-100]' })).toBeInTheDocument();
      expect(screen.getByRole('checkbox', { name: 'B [50-74]' })).toBeInTheDocument();
      expect(screen.getByRole('checkbox', { name: 'C [25-49]' })).toBeInTheDocument();
      expect(screen.getByRole('checkbox', { name: 'D [0-24]' })).toBeInTheDocument();

      expect(screen.getByText('Category')).toBeInTheDocument();
      expect(screen.getByRole('checkbox', { name: 'App definition' })).toBeInTheDocument();
      expect(screen.getByRole('checkbox', { name: 'Observability' })).toBeInTheDocument();
      expect(screen.getByRole('checkbox', { name: 'Orchestration' })).toBeInTheDocument();
      expect(screen.getByRole('checkbox', { name: 'Platform' })).toBeInTheDocument();
      expect(screen.getByRole('checkbox', { name: 'Provisioning' })).toBeInTheDocument();
      expect(screen.getByRole('checkbox', { name: 'Runtime' })).toBeInTheDocument();
      expect(screen.getByRole('checkbox', { name: 'Serverless' })).toBeInTheDocument();

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

    it('renders Filters with selected options', () => {
      render(<Filters {...defaultProps} activeFilters={{ maturity: ['2'], rating: ['a', 'b'] }} />);

      expect(screen.getByRole('checkbox', { name: 'Sandbox' })).toBeChecked();
      expect(screen.getByRole('checkbox', { name: 'A [75-100]' })).toBeChecked();
      expect(screen.getByRole('checkbox', { name: 'B [50-74]' })).toBeChecked();
    });

    it('calls onChange to click filter', () => {
      render(<Filters {...defaultProps} />);

      const check = screen.getByRole('checkbox', { name: 'App definition' });

      expect(check).not.toBeChecked();

      userEvent.click(check);

      expect(mockOnChange).toHaveBeenCalledTimes(1);
      expect(mockOnChange).toHaveBeenCalledWith('category', '0', true);
    });
  });
});
