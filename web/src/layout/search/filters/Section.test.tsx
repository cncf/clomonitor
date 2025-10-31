import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { vi } from 'vitest';

import { FilterKind, Rating } from '../../../types';
import Section from './Section';

const mockOnChange = vi.fn();

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
    vi.resetAllMocks();
  });

  it('creates snapshot', () => {
    const { asFragment } = render(<Section {...defaultProps} />);

    expect(asFragment()).toMatchSnapshot();
  });

  describe('Render', () => {
    it('renders Section', () => {
      render(<Section {...defaultProps} />);

      expect(screen.getByText('Rating')).toBeInTheDocument();
      expect(screen.getByRole('checkbox', { name: /A/ })).toBeInTheDocument();
      expect(screen.getByRole('checkbox', { name: /B/ })).toBeInTheDocument();
      expect(screen.getByRole('checkbox', { name: /C/ })).toBeInTheDocument();
      expect(screen.getByRole('checkbox', { name: /D/ })).toBeInTheDocument();
    });

    it('renders Section with selected options', () => {
      render(<Section {...defaultProps} activeFilters={['a', 'b']} />);

      expect(screen.getByRole('checkbox', { name: /A/ })).toBeChecked();
      expect(screen.getByRole('checkbox', { name: /B/ })).toBeChecked();
    });

    it('calls onChange to click filter', async () => {
      render(<Section {...defaultProps} />);

      const check = screen.getByRole('checkbox', { name: /B/ });

      expect(check).not.toBeChecked();

      await userEvent.click(check);

      expect(mockOnChange).toHaveBeenCalledTimes(1);
      expect(mockOnChange).toHaveBeenCalledWith('rating', 'b', true);
    });

    it('calls onChange to click selected filter', async () => {
      render(<Section {...defaultProps} activeFilters={['b']} />);

      const check = screen.getByRole('checkbox', { name: /B/ });

      expect(check).toBeChecked();

      await userEvent.click(check);

      expect(mockOnChange).toHaveBeenCalledTimes(1);
      expect(mockOnChange).toHaveBeenCalledWith('rating', 'b', false);
    });
  });
});
