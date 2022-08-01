import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';

import Settings from './Settings';

describe('Settings', () => {
  afterEach(() => {
    jest.resetAllMocks();
  });

  it('creates snapshot', () => {
    const { asFragment } = render(<Settings />);
    expect(asFragment()).toMatchSnapshot();
  });

  it('renders proper content', () => {
    render(<Settings />);

    expect(screen.getByRole('button', { name: 'Settings button' })).toBeInTheDocument();
  });

  it('opens dropdown', async () => {
    render(<Settings />);

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
    render(<Settings />);

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
