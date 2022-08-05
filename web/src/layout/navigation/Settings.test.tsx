import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';

import { AppContext } from '../../context/AppContextProvider';
import { SortBy, SortDirection } from '../../types';
import Settings from './Settings';

const mockCtx = {
  prefs: {
    search: { limit: 20, sort: { by: SortBy.Name, direction: SortDirection.ASC } },
    theme: { effective: 'light', configured: 'light' },
  },
};

describe('Settings', () => {
  afterEach(() => {
    jest.resetAllMocks();
  });

  it('creates snapshot', () => {
    const { asFragment } = render(
      <AppContext.Provider value={{ ctx: mockCtx, dispatch: jest.fn() }}>
        <Settings />
      </AppContext.Provider>
    );
    expect(asFragment()).toMatchSnapshot();
  });

  it('renders proper content', () => {
    render(
      <AppContext.Provider value={{ ctx: mockCtx, dispatch: jest.fn() }}>
        <Settings />
      </AppContext.Provider>
    );

    expect(screen.getByRole('button', { name: 'Settings button' })).toBeInTheDocument();
  });

  it('opens dropdown', async () => {
    render(
      <AppContext.Provider value={{ ctx: mockCtx, dispatch: jest.fn() }}>
        <Settings />
      </AppContext.Provider>
    );

    const dropdown = screen.getByRole('menu');
    expect(dropdown).toBeInTheDocument();
    expect(dropdown).not.toHaveClass('show');

    const btn = screen.getByRole('button', { name: 'Settings button' });
    await userEvent.click(btn);

    expect(dropdown).toHaveClass('show');
    expect(screen.getByRole('radio', { name: 'Automatic' })).toBeInTheDocument();
    expect(screen.getByRole('radio', { name: 'Automatic' })).not.toBeChecked();
    expect(screen.getByRole('radio', { name: 'Light' })).toBeInTheDocument();
    expect(screen.getByRole('radio', { name: 'Light' })).toBeChecked();
    expect(screen.getByRole('radio', { name: 'Dark' })).toBeInTheDocument();
    expect(screen.getByRole('radio', { name: 'Dark' })).not.toBeChecked();
  });

  it('opens dropdown and closes it using Settings button', async () => {
    render(
      <AppContext.Provider value={{ ctx: mockCtx, dispatch: jest.fn() }}>
        <Settings />
      </AppContext.Provider>
    );

    const dropdown = screen.getByRole('menu');
    expect(dropdown).toBeInTheDocument();
    expect(dropdown).not.toHaveClass('show');

    const btn = screen.getByRole('button', { name: 'Settings button' });
    await userEvent.click(btn);

    expect(dropdown).toHaveClass('show');
    expect(screen.getByRole('radio', { name: 'Automatic' })).toBeInTheDocument();
    expect(screen.getByRole('radio', { name: 'Automatic' })).not.toBeChecked();
    expect(screen.getByRole('radio', { name: 'Light' })).toBeInTheDocument();
    expect(screen.getByRole('radio', { name: 'Light' })).toBeChecked();
    expect(screen.getByRole('radio', { name: 'Dark' })).toBeInTheDocument();
    expect(screen.getByRole('radio', { name: 'Dark' })).not.toBeChecked();

    await userEvent.click(btn);

    expect(dropdown).not.toHaveClass('show');
  });
});
