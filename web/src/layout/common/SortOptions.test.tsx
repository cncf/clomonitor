import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';

import { SortBy, SortDirection } from '../../types';
import SortOptions from './SortOptions';

const mockOnSortChange = jest.fn();

const defaultProps = {
  by: SortBy.Name,
  direction: SortDirection.ASC,
  onSortChange: mockOnSortChange,
};

describe('SortOptions', () => {
  afterEach(() => {
    jest.resetAllMocks();
  });

  it('creates snapshot', () => {
    const { asFragment } = render(<SortOptions {...defaultProps} />);
    expect(asFragment()).toMatchSnapshot();
  });

  it('renders proper content', () => {
    render(<SortOptions {...defaultProps} />);

    expect(screen.getByText('Sort:')).toBeInTheDocument();

    expect(screen.getByRole('combobox')).toBeInTheDocument();
    expect(screen.getAllByRole('option').length).toBe(4);
    expect(screen.getByRole('option', { name: 'Alphabetically (A-Z)' })).toBeInTheDocument();
    expect(screen.getByRole('option', { name: 'Alphabetically (Z-A)' })).toBeInTheDocument();
    expect(screen.getByRole('option', { name: 'Score (highest first)' })).toBeInTheDocument();
    expect(screen.getByRole('option', { name: 'Score (lowest first)' })).toBeInTheDocument();
    expect((screen.getByRole('option', { name: 'Alphabetically (A-Z)' }) as HTMLOptionElement).selected).toBe(true);

    expect(screen.getByLabelText('Sort options select')).toBeInTheDocument();
    expect(screen.getByLabelText('Sort options select')).toHaveValue('name_asc');
  });

  it('calls onChange to update select', () => {
    render(<SortOptions {...defaultProps} />);

    userEvent.selectOptions(
      screen.getByRole('combobox'),
      screen.getByRole('option', { name: 'Score (highest first)' })
    );

    expect(mockOnSortChange).toHaveBeenCalledTimes(1);
    expect(mockOnSortChange).toHaveBeenCalledWith('score', 'desc');
  });
});
