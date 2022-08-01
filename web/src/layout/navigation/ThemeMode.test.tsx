import { fireEvent, render, screen } from '@testing-library/react';
import { BrowserRouter as Router } from 'react-router-dom';

import { AppContext } from '../../context/AppContextProvider';
import { SortBy, SortDirection } from '../../types';
import ThemeMode from './ThemeMode';

const mockCtx = {
  prefs: {
    search: { limit: 20, sort: { by: SortBy.Name, direction: SortDirection.ASC } },
    theme: { effective: 'light', configured: 'light' },
  },
};

const mockDispatch = jest.fn();
const mockOnChange = jest.fn();

const defaultProps = {
  device: 'desktop',
  onChange: mockOnChange,
};

describe('ThemeMode', () => {
  afterEach(() => {
    jest.resetAllMocks();
  });

  it('creates snapshot', () => {
    const { asFragment } = render(
      <AppContext.Provider value={{ ctx: mockCtx, dispatch: jest.fn() }}>
        <Router>
          <ThemeMode {...defaultProps} />
        </Router>
      </AppContext.Provider>
    );
    expect(asFragment()).toMatchSnapshot();
  });

  it('renders proper content', () => {
    render(
      <AppContext.Provider value={{ ctx: mockCtx, dispatch: jest.fn() }}>
        <Router>
          <ThemeMode {...defaultProps} />
        </Router>
      </AppContext.Provider>
    );

    expect(screen.getByRole('radio', { name: 'Automatic' })).toBeInTheDocument();
    expect(screen.getByRole('radio', { name: 'Light' })).toBeInTheDocument();
    expect(screen.getByRole('radio', { name: 'Dark' })).toBeInTheDocument();
  });

  it('calls updateTheme event', async () => {
    render(
      <AppContext.Provider value={{ ctx: mockCtx, dispatch: mockDispatch }}>
        <Router>
          <ThemeMode {...defaultProps} />
        </Router>
      </AppContext.Provider>
    );

    fireEvent.click(screen.getByRole('radio', { name: 'Dark' }));

    expect(mockDispatch).toHaveBeenCalledTimes(1);
    expect(mockDispatch).toHaveBeenCalledWith({ theme: 'dark', type: 'updateTheme' });
  });
});
