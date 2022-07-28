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

  it('opens dropdown', async () => {
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
    await userEvent.click(btn);

    expect(dropdown).toHaveClass('show');
    expect(screen.getByRole('radio', { name: 'Light' })).toBeInTheDocument();
    expect(screen.getByRole('radio', { name: 'Light' })).toBeChecked();
    expect(screen.getByRole('radio', { name: 'Dark' })).toBeInTheDocument();
    expect(screen.getByRole('radio', { name: 'Dark' })).not.toBeChecked();
    expect(screen.getByText('Statistics')).toBeInTheDocument();
    expect(screen.getAllByRole('link')).toHaveLength(2);
  });

  it('opens dropdown and closes it using Settings button', async () => {
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
    await userEvent.click(btn);

    expect(dropdown).toHaveClass('show');
    expect(screen.getByRole('radio', { name: 'Light' })).toBeInTheDocument();
    expect(screen.getByRole('radio', { name: 'Light' })).toBeChecked();
    expect(screen.getByRole('radio', { name: 'Dark' })).toBeInTheDocument();
    expect(screen.getByRole('radio', { name: 'Dark' })).not.toBeChecked();
    expect(screen.getByText('Statistics')).toBeInTheDocument();
    expect(screen.getAllByRole('link')).toHaveLength(2);

    await userEvent.click(btn);

    expect(dropdown).not.toHaveClass('show');
  });

  it('calls updateTheme event', async () => {
    render(
      <AppContext.Provider value={{ ctx: mockCtx, dispatch: mockDispatch }}>
        <Router>
          <MobileSettings />
        </Router>
      </AppContext.Provider>
    );

    const btn = screen.getByRole('button', { name: 'Mobile settings button' });
    await userEvent.click(btn);

    fireEvent.click(screen.getByRole('radio', { name: 'Dark' }));

    expect(mockDispatch).toHaveBeenCalledTimes(1);
    expect(mockDispatch).toHaveBeenCalledWith({ theme: 'dark', type: 'updateTheme' });
  });

  it('goes to Docs page', async () => {
    render(
      <AppContext.Provider value={{ ctx: mockCtx, dispatch: mockDispatch }}>
        <Router>
          <MobileSettings />
        </Router>
      </AppContext.Provider>
    );

    const btn = screen.getByRole('button', { name: 'Mobile settings button' });
    await userEvent.click(btn);

    const link = screen.getByRole('link', { name: 'Open documentation' });
    expect(link).toBeInTheDocument();
    expect(link).toHaveAttribute('href', '/docs');
  });

  it('goes to Stats page', async () => {
    render(
      <AppContext.Provider value={{ ctx: mockCtx, dispatch: mockDispatch }}>
        <Router>
          <MobileSettings />
        </Router>
      </AppContext.Provider>
    );

    const btn = screen.getByRole('button', { name: 'Mobile settings button' });
    await userEvent.click(btn);

    const links = screen.getAllByRole('link');
    fireEvent.click(links[1]);

    expect(window.location.pathname).toBe('/stats');
  });
});
