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

const realDate = Date;

const freezeDate = (isoDate: string) => {
  const fixedDate = new realDate(isoDate);

  const MockDate = class extends realDate {
    constructor(value?: number | string | Date) {
      if (arguments.length === 0) {
        return new realDate(fixedDate);
      }
      return new realDate(value as number | string | Date);
    }

    static now(): number {
      return fixedDate.getTime();
    }

    static parse(dateString: string): number {
      return realDate.parse(dateString);
    }

    static UTC(...args: Parameters<typeof realDate.UTC>): number {
      return realDate.UTC(...args);
    }
  };

  Object.setPrototypeOf(MockDate, realDate);
  // @ts-expect-error overriding global Date for tests
  global.Date = MockDate as unknown as DateConstructor;
};

describe('Filters', () => {
  beforeEach(() => {
    freezeDate('2022-03-24T00:00:00.000Z');
  });

  afterEach(() => {
    // @ts-expect-error restoring original Date
    global.Date = realDate;
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
