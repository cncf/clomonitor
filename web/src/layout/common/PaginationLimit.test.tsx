import { fireEvent, render, screen } from '@testing-library/react';

import { AppContext } from '../../context/AppContextProvider';
import { SortBy, SortDirection } from '../../types';
import PaginationLimit from './PaginationLimit';

const mockOnPaginationLimitChange = jest.fn();

const defaultProps = {
  onPaginationLimitChange: mockOnPaginationLimitChange,
};

const mockCtx = {
  prefs: {
    search: { limit: 20, sort: { by: SortBy.Name, direction: SortDirection.ASC } },
    theme: { effective: 'light', configured: 'light' },
  },
};

describe('PaginationLimit', () => {
  afterEach(() => {
    jest.resetAllMocks();
  });

  it('creates snapshot', () => {
    const { asFragment } = render(
      <AppContext.Provider value={{ ctx: mockCtx, dispatch: jest.fn() }}>
        <PaginationLimit {...defaultProps} />
      </AppContext.Provider>
    );
    expect(asFragment()).toMatchSnapshot();
  });

  it('renders proper content', () => {
    render(
      <AppContext.Provider value={{ ctx: mockCtx, dispatch: jest.fn() }}>
        <PaginationLimit {...defaultProps} />
      </AppContext.Provider>
    );

    expect(screen.getByText('Show:')).toBeInTheDocument();
    expect(screen.getByRole('combobox')).toBeInTheDocument();
    expect(screen.getAllByRole('option').length).toBe(3);
    expect(screen.getByRole('option', { name: '20' })).toBeInTheDocument();
    expect(screen.getByRole('option', { name: '40' })).toBeInTheDocument();
    expect(screen.getByRole('option', { name: '60' })).toBeInTheDocument();
    expect((screen.getByRole('option', { name: '20' }) as HTMLOptionElement).selected).toBe(true);

    expect(screen.getByLabelText('Pagination limit select')).toBeInTheDocument();
    expect(screen.getByLabelText('Pagination limit select')).toHaveValue('20');
  });

  it('renders limit from context', () => {
    render(
      <AppContext.Provider
        value={{
          ctx: { prefs: { ...mockCtx.prefs, search: { ...mockCtx.prefs.search, limit: 60 } } },
          dispatch: jest.fn(),
        }}
      >
        <PaginationLimit {...defaultProps} />
      </AppContext.Provider>
    );

    expect((screen.getByRole('option', { name: '60' }) as HTMLOptionElement).selected).toBe(true);
  });

  it('calls onChange to update select', () => {
    render(
      <AppContext.Provider value={{ ctx: mockCtx, dispatch: jest.fn() }}>
        <PaginationLimit {...defaultProps} />
      </AppContext.Provider>
    );

    const select = screen.getByRole('combobox');
    fireEvent.change(select, '40');

    expect(mockOnPaginationLimitChange).toHaveBeenCalledTimes(1);
    expect(mockOnPaginationLimitChange).toHaveBeenCalledWith(20);
  });
});
