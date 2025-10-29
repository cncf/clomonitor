import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { vi } from 'vitest';

import SelectedFilters from './SelectedFilters';

const mockOnChange = vi.fn();
const mockOnAcceptedDateRangeChange = vi.fn();

const defaultProps = {
  filters: { maturity: ['sandbox'], rating: ['a', 'b'], foundation: ['cncf'] },
  acceptedFrom: '2018-01-01',
  onChange: mockOnChange,
  onAcceptedDateRangeChange: mockOnAcceptedDateRangeChange,
};

describe('SelectedFilters', () => {
  afterEach(() => {
    vi.resetAllMocks();
  });

  it('creates snapshot', () => {
    const { asFragment } = render(<SelectedFilters {...defaultProps} />);
    expect(asFragment()).toMatchSnapshot();
  });

  describe('Render', () => {
    it('renders component', () => {
      render(<SelectedFilters {...defaultProps} />);

      expect(screen.getByText('Filters:')).toBeInTheDocument();
      expect(screen.getByRole('list')).toBeInTheDocument();
      expect(screen.getAllByRole('listitem')).toHaveLength(5);

      expect(screen.getByText('Accepted:')).toBeInTheDocument();
      expect(screen.getByText('≥ Jan 1, 2018')).toBeInTheDocument();
      expect(screen.getByText('Maturity:')).toBeInTheDocument();
      expect(screen.getByText('Sandbox')).toBeInTheDocument();
      expect(screen.getAllByText('Rating:')).toHaveLength(2);
      expect(screen.getByText('A [75-100]')).toBeInTheDocument();
      expect(screen.getByText('B [50-74]')).toBeInTheDocument();
      expect(screen.getByText('Foundation:')).toBeInTheDocument();
      expect(screen.getByText('CNCF')).toBeInTheDocument();

      expect(screen.getAllByRole('button')).toHaveLength(5);
      expect(screen.getByRole('button', { name: 'Remove Sandbox filter' })).toBeInTheDocument();
      expect(screen.getByRole('button', { name: 'Remove A [75-100] filter' })).toBeInTheDocument();
      expect(screen.getByRole('button', { name: 'Remove B [50-74] filter' })).toBeInTheDocument();
      expect(screen.getByRole('button', { name: 'Remove CNCF filter' })).toBeInTheDocument();
      expect(screen.getByRole('button', { name: 'Remove accepted filter' })).toBeInTheDocument();
    });

    it('calls on change', async () => {
      render(<SelectedFilters {...defaultProps} />);

      const btn = screen.getByRole('button', { name: 'Remove B [50-74] filter' });
      await userEvent.click(btn);

      expect(mockOnChange).toHaveBeenCalledTimes(1);
      expect(mockOnChange).toHaveBeenCalledWith('rating', 'b', false);
    });
  });

  describe('renders proper Accepted tag filter', () => {
    it('when both dates are defined', () => {
      render(<SelectedFilters {...defaultProps} acceptedTo="2021-12-31" />);

      expect(screen.getByText('Accepted:')).toBeInTheDocument();
      expect(screen.getByText('Jan 1, 2018 - Dec 31, 2021')).toBeInTheDocument();
    });

    it('when both dates are defined and are the same year', () => {
      render(<SelectedFilters {...defaultProps} acceptedTo="2018-12-31" />);

      expect(screen.getByText('Accepted:')).toBeInTheDocument();
      expect(screen.getByText('Jan 1, 2018 - Dec 31, 2018')).toBeInTheDocument();
    });

    it('only accepted_from is defined', () => {
      render(<SelectedFilters {...defaultProps} />);

      expect(screen.getByText('Accepted:')).toBeInTheDocument();
      expect(screen.getByText('≥ Jan 1, 2018')).toBeInTheDocument();
    });

    it('only accepted_to is defined', () => {
      render(<SelectedFilters {...defaultProps} acceptedFrom={undefined} acceptedTo="2021-12-31" />);

      expect(screen.getByText('Accepted:')).toBeInTheDocument();
      expect(screen.getByText('≤ Dec 31, 2021')).toBeInTheDocument();
    });
  });

  describe('Does not render', () => {
    it('when filters is empty', () => {
      const { container } = render(<SelectedFilters {...defaultProps} acceptedFrom={undefined} filters={{}} />);
      expect(container).toBeEmptyDOMElement();
    });
  });
});
