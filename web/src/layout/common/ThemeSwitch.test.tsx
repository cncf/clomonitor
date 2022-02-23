import { fireEvent, render, screen } from '@testing-library/react';

import { AppContext } from '../../context/AppContextProvider';
import { SortBy, SortDirection } from '../../types';
import ThemeSwitch from './ThemeSwitch';

const mockCtx = {
  prefs: {
    search: { limit: 20, sort: { by: SortBy.Name, direction: SortDirection.ASC } },
    theme: { effective: 'light' },
  },
};

const mockDispatch = jest.fn();

describe('ThemeSwitch', () => {
  afterEach(() => {
    jest.resetAllMocks();
  });

  it('creates snapshot', () => {
    const { asFragment } = render(
      <AppContext.Provider value={{ ctx: mockCtx, dispatch: jest.fn() }}>
        <ThemeSwitch />
      </AppContext.Provider>
    );
    expect(asFragment()).toMatchSnapshot();
  });

  it('renders proper content', () => {
    render(
      <AppContext.Provider value={{ ctx: mockCtx, dispatch: jest.fn() }}>
        <ThemeSwitch />
      </AppContext.Provider>
    );

    expect(screen.getByRole('switch')).toBeInTheDocument();
    expect(screen.getByRole('switch')).toBeChecked();

    expect(screen.getByTestId('sun-icon')).toBeInTheDocument();
    expect(screen.queryByTestId('moon-icon')).toBeNull();
  });

  it('renders effective theme from context', () => {
    render(
      <AppContext.Provider
        value={{
          ctx: { prefs: { ...mockCtx.prefs, theme: { effective: 'dark' } } },
          dispatch: jest.fn(),
        }}
      >
        <ThemeSwitch />
      </AppContext.Provider>
    );

    expect(screen.getByRole('switch')).not.toBeChecked();

    expect(screen.getByTestId('moon-icon')).toBeInTheDocument();
    expect(screen.queryByTestId('sun-icon')).toBeNull();
  });

  it('calls updateTheme event', () => {
    render(
      <AppContext.Provider value={{ ctx: mockCtx, dispatch: mockDispatch }}>
        <ThemeSwitch />
      </AppContext.Provider>
    );

    fireEvent.click(screen.getByRole('switch'));

    expect(mockDispatch).toHaveBeenCalledTimes(1);
    expect(mockDispatch).toHaveBeenCalledWith({ theme: 'dark', type: 'updateTheme' });
  });
});
