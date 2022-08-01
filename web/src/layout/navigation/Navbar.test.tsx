import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { BrowserRouter as Router } from 'react-router-dom';

import Navbar from './Navbar';

const mockSetScrollPosition = jest.fn();

const defaultProps = {
  setScrollPosition: mockSetScrollPosition,
};

describe('Navbar', () => {
  afterEach(() => {
    jest.resetAllMocks();
  });

  it('creates snapshot', () => {
    const { asFragment } = render(
      <Router>
        <Navbar {...defaultProps} />
      </Router>
    );
    expect(asFragment()).toMatchSnapshot();
  });

  it('renders proper content', () => {
    render(
      <Router>
        <Navbar {...defaultProps} />
      </Router>
    );

    expect(screen.getByText('Beta')).toBeInTheDocument();
    expect(screen.getByAltText('CLOMonitor logo')).toBeInTheDocument();

    const links = screen.getAllByRole('link');
    expect(links[0]).toBeInTheDocument();
    expect(links[0]).toHaveAttribute('href', '/');

    expect(screen.getByRole('textbox')).toBeInTheDocument();

    expect(screen.getAllByRole('radio', { name: 'Automatic' })).toHaveLength(2);
    expect(screen.getAllByRole('radio', { name: 'Light' })).toHaveLength(2);
    expect(screen.getAllByRole('radio', { name: 'Dark' })).toHaveLength(2);
  });

  it('clicks logo', async () => {
    render(
      <Router>
        <Navbar {...defaultProps} />
      </Router>
    );

    const links = screen.getAllByRole('link');
    await userEvent.click(links[0]);

    expect(mockSetScrollPosition).toHaveBeenCalledTimes(1);
    expect(mockSetScrollPosition).toHaveBeenCalledWith(0);

    expect(window.location.pathname).toBe('/');
  });

  it('clicks Docs page', () => {
    render(
      <Router>
        <Navbar {...defaultProps} />
      </Router>
    );

    const links = screen.getAllByRole('link', { name: 'Open documentation' });
    expect(links).toHaveLength(2);
    expect(links[0]).toHaveAttribute('href', '/docs');
  });

  it('clicks Stats page', async () => {
    render(
      <Router>
        <Navbar {...defaultProps} />
      </Router>
    );

    const links = screen.getAllByRole('link');
    await userEvent.click(links[2]);

    expect(window.location.pathname).toBe('/stats');
  });
});
