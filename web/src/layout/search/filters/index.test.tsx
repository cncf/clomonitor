import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { vi } from 'vitest';

import Filters from './index';

const mockOnChange = vi.fn();
const mockOnChecksChange = vi.fn();
const mockOnAcceptedDateRangeChange = vi.fn();

const defaultProps = {
  visibleTitle: true,
  activeFilters: {},
  onChange: mockOnChange,
  onChecksChange: mockOnChecksChange,
  onAcceptedDateRangeChange: mockOnAcceptedDateRangeChange,
  device: 'test',
};

describe('Filters', () => {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  let dateNowSpy: any;

  beforeEach(() => {
    dateNowSpy = vi.spyOn(Date, 'now').mockImplementation(() => 1648154630000);
  });

  afterAll(() => {
    dateNowSpy.mockRestore();
  });

  afterEach(() => {
    vi.resetAllMocks();
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

      expect(screen.getByText('Rating')).toBeInTheDocument();
      expect(screen.getByRole('checkbox', { name: /A\s?\[75-100]/i })).toBeInTheDocument();
      expect(screen.getByRole('checkbox', { name: /B\s?\[50-74]/i })).toBeInTheDocument();
      expect(screen.getByRole('checkbox', { name: /C\s?\[25-49]/i })).toBeInTheDocument();
      expect(screen.getByRole('checkbox', { name: /D\s?\[0-24]/i })).toBeInTheDocument();

      expect(screen.getAllByText('From:')).toHaveLength(2);
      expect(screen.getAllByText('To:')).toHaveLength(2);
      expect(screen.getByText('Jan 1, 2016')).toBeInTheDocument();
      expect(screen.getByText('Mar 24, 2022')).toBeInTheDocument();

      expect(screen.getByText('Checks')).toBeInTheDocument();
      expect(screen.getByRole('button', { name: 'Open checks modal' })).toHaveTextContent('Add checks filters');
    });

    it('renders Filters', () => {
      render(<Filters {...defaultProps} activeFilters={{ foundation: ['cncf'] }} />);

      expect(screen.getByText('Filters')).toBeInTheDocument();

      expect(screen.getByText('Foundation')).toBeInTheDocument();
      expect(screen.getByRole('checkbox', { name: 'CNCF' })).toBeChecked();

      expect(screen.getByText('Maturity level')).toBeInTheDocument();
      expect(screen.getByRole('checkbox', { name: 'Graduated' })).toBeInTheDocument();
      expect(screen.getByRole('checkbox', { name: 'Incubating' })).toBeInTheDocument();
      expect(screen.getByRole('checkbox', { name: 'Sandbox' })).toBeInTheDocument();
    });

    it('renders Filters with selected options', () => {
      render(<Filters {...defaultProps} activeFilters={{ foundation: ['cncf'], rating: ['a', 'b'] }} />);

      expect(screen.getByRole('checkbox', { name: 'CNCF' })).toBeChecked();
      expect(screen.getByRole('checkbox', { name: /A\s?\[75-100]/i })).toBeChecked();
      expect(screen.getByRole('checkbox', { name: /B\s?\[50-74]/i })).toBeChecked();
    });

    it('calls onChange to click filter', async () => {
      render(<Filters {...defaultProps} />);

      const check = screen.getByRole('checkbox', { name: /A\s?\[75-100]/i });

      expect(check).not.toBeChecked();

      await userEvent.click(check);

      expect(mockOnChange).toHaveBeenCalledTimes(1);
      expect(mockOnChange).toHaveBeenCalledWith('rating', 'a', true);
    });
  });
});
