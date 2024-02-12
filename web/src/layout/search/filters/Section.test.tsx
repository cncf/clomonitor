import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';

import { FilterKind, Rating } from '../../../types';
import Section from './Section';

const mockOnChange = jest.fn();

const defaultProps = {
  section: {
    name: FilterKind.Rating,
    title: 'Rating',
    filters: [
      {
        name: Rating.A,
        label: 'A',
        legend: '[75-100]',
      },
      {
        name: Rating.B,
        label: 'B',
        legend: '[50-74]',
      },
      {
        name: Rating.C,
        label: 'C',
        legend: '[25-49]',
      },
      {
        name: Rating.D,
        label: 'D',
        legend: '[0-24]',
      },
    ],
  },
  activeFilters: [],
  onChange: mockOnChange,
  device: 'test',
};

describe('Section', () => {
  afterEach(() => {
    jest.resetAllMocks();
  });

  it('creates snapshot', () => {
    const { asFragment } = render(<Section {...defaultProps} />);

    expect(asFragment()).toMatchSnapshot();
  });

  describe('Render', () => {
    it('renders Section', () => {
      render(<Section {...defaultProps} />);

      expect(screen.getByText('Rating')).toBeInTheDocument();
      expect(screen.getByRole('checkbox', { name: 'A [75-100]' })).toBeInTheDocument();
      expect(screen.getByRole('checkbox', { name: 'B [50-74]' })).toBeInTheDocument();
      expect(screen.getByRole('checkbox', { name: 'C [25-49]' })).toBeInTheDocument();
      expect(screen.getByRole('checkbox', { name: 'D [0-24]' })).toBeInTheDocument();
    });

    it('renders Section with selected options', () => {
      render(<Section {...defaultProps} activeFilters={['a', 'b']} />);

      expect(screen.getByRole('checkbox', { name: 'A [75-100]' })).toBeChecked();
      expect(screen.getByRole('checkbox', { name: 'B [50-74]' })).toBeChecked();
    });

    it('calls onChange to click filter', async () => {
      render(<Section {...defaultProps} />);

      const check = screen.getByRole('checkbox', { name: 'B [50-74]' });

      expect(check).not.toBeChecked();

      await userEvent.click(check);

      expect(mockOnChange).toHaveBeenCalledTimes(1);
      expect(mockOnChange).toHaveBeenCalledWith('rating', 'b', true);
    });

    it('calls onChange to click selected filter', async () => {
      render(<Section {...defaultProps} activeFilters={['b']} />);

      const check = screen.getByRole('checkbox', { name: 'B [50-74]' });

      expect(check).toBeChecked();

      await userEvent.click(check);

      expect(mockOnChange).toHaveBeenCalledTimes(1);
      expect(mockOnChange).toHaveBeenCalledWith('rating', 'b', false);
    });
  });
});
