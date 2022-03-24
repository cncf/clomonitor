import { fireEvent, render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { BrowserRouter as Router } from 'react-router-dom';

import { AppContext } from '../../context/AppContextProvider';
import { SortBy, SortDirection } from '../../types';
import MobileSettings from './MobileSettings';

const mockCtx = {
  prefs: {
    search: { limit: 20, sort: { by: SortBy.Name, direction: SortDirection.ASC } },
    theme: { effective: 'light' },
  },
};
const mockDispatch = jest.fn();

describe('MobileSettings', () => {
  afterEach(() => {
    jest.resetAllMocks();
  });

  it('creates snapshot', () => {
    const { asFragment } = render(
      <AppContext.Provider value={{ ctx: mockCtx, dispatch: jest.fn() }}>
        <Router>
          <MobileSettings />
        </Router>
      </AppContext.Provider>
    );
    expect(asFragment()).toMatchSnapshot();
  });

  it('renders proper content', () => {
    render(
      <AppContext.Provider value={{ ctx: mockCtx, dispatch: jest.fn() }}>
        <Router>
          <MobileSettings />
        </Router>
      </AppContext.Provider>
    );

    expect(screen.getByRole('button', { name: 'Mobile settings button' })).toBeInTheDocument();
  });

  it('opens dropdown', () => {
    render(
      <AppContext.Provider value={{ ctx: mockCtx, dispatch: jest.fn() }}>
        <Router>
          <MobileSettings />
        </Router>
      </AppContext.Provider>
    );

    const dropdown = screen.getByRole('menu');
    expect(dropdown).toBeInTheDocument();
    expect(dropdown).not.toHaveClass('show');

    const btn = screen.getByRole('button', { name: 'Mobile settings button' });
    userEvent.click(btn);

    expect(dropdown).toHaveClass('show');
    expect(screen.getByRole('radio', { name: 'Light' })).toBeInTheDocument();
    expect(screen.getByRole('radio', { name: 'Light' })).toBeChecked();
    expect(screen.getByRole('radio', { name: 'Dark' })).toBeInTheDocument();
    expect(screen.getByRole('radio', { name: 'Dark' })).not.toBeChecked();
    expect(screen.getByText('Statistics')).toBeInTheDocument();
    expect(screen.getByRole('link')).toBeInTheDocument();
  });

  it('calls updateTheme event', () => {
    render(
      <AppContext.Provider value={{ ctx: mockCtx, dispatch: mockDispatch }}>
        <Router>
          <MobileSettings />
        </Router>
      </AppContext.Provider>
    );

    const btn = screen.getByRole('button', { name: 'Mobile settings button' });
    userEvent.click(btn);

    fireEvent.click(screen.getByRole('radio', { name: 'Dark' }));

    expect(mockDispatch).toHaveBeenCalledTimes(1);
    expect(mockDispatch).toHaveBeenCalledWith({ theme: 'dark', type: 'updateTheme' });
  });

  it('goes to Stats page', () => {
    render(
      <AppContext.Provider value={{ ctx: mockCtx, dispatch: mockDispatch }}>
        <Router>
          <MobileSettings />
        </Router>
      </AppContext.Provider>
    );

    const btn = screen.getByRole('button', { name: 'Mobile settings button' });
    userEvent.click(btn);

    fireEvent.click(screen.getByRole('link'));

    expect(window.location.pathname).toBe('/stats');
  });
});
