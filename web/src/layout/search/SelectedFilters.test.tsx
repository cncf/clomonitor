import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';

import SelectedFilters from './SelectedFilters';

const mockOnChange = jest.fn();

const defaultProps = {
  filters: { maturity: ['2'], rating: ['a', 'b'], category: ['0', '1'] },
  onChange: mockOnChange,
};

describe('SelectedFilters', () => {
  afterEach(() => {
    jest.resetAllMocks();
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

      expect(screen.getByText('maturity:')).toBeInTheDocument();
      expect(screen.getByText('Sandbox')).toBeInTheDocument();
      expect(screen.getAllByText('rating:')).toHaveLength(2);
      expect(screen.getByText('A [75-100]')).toBeInTheDocument();
      expect(screen.getByText('B [50-74]')).toBeInTheDocument();
      expect(screen.getAllByText('category:')).toHaveLength(2);
      expect(screen.getByText('App definition')).toBeInTheDocument();
      expect(screen.getByText('Observability')).toBeInTheDocument();

      expect(screen.getAllByRole('button')).toHaveLength(5);
      expect(screen.getByRole('button', { name: 'Remove Sandbox filter' })).toBeInTheDocument();
      expect(screen.getByRole('button', { name: 'Remove A [75-100] filter' })).toBeInTheDocument();
      expect(screen.getByRole('button', { name: 'Remove B [50-74] filter' })).toBeInTheDocument();
      expect(screen.getByRole('button', { name: 'Remove App definition filter' })).toBeInTheDocument();
      expect(screen.getByRole('button', { name: 'Remove Observability filter' })).toBeInTheDocument();
    });

    it('calls on change', () => {
      render(<SelectedFilters {...defaultProps} />);

      const btn = screen.getByRole('button', { name: 'Remove B [50-74] filter' });
      userEvent.click(btn);

      expect(mockOnChange).toHaveBeenCalledTimes(1);
      expect(mockOnChange).toHaveBeenCalledWith('rating', 'b', false);
    });
  });

  describe('Does not render', () => {
    it('when filters is empty', () => {
      const { container } = render(<SelectedFilters {...defaultProps} filters={{}} />);
      expect(container).toBeEmptyDOMElement();
    });
  });
});
