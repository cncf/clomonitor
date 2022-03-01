import { render, screen } from '@testing-library/react';

import Footer from './Footer';

describe('Footer', () => {
  afterEach(() => {
    jest.resetAllMocks();
  });

  it('creates snapshot', () => {
    const { asFragment } = render(<Footer />);
    expect(asFragment()).toMatchSnapshot();
  });

  it('renders proper content', () => {
    render(<Footer />);

    expect(screen.getByText('Project')).toBeInTheDocument();
    expect(screen.getByText('Community')).toBeInTheDocument();
    expect(screen.getByText('About')).toBeInTheDocument();
    expect(screen.getByText(/CLOMonitor is an/)).toBeInTheDocument();
    expect(screen.getByText('Open Source')).toBeInTheDocument();

    const links = screen.getAllByRole('link');
    expect(links).toHaveLength(3);
    expect(links[0]).toHaveTextContent('Documentation');
    expect(links[0]).toHaveAttribute('href', 'https://github.com/cncf/clomonitor/tree/main/docs');
    expect(links[1]).toHaveTextContent('GitHub');
    expect(links[1]).toHaveAttribute('href', 'https://github.com/cncf/clomonitor');
    expect(links[2]).toHaveTextContent('Apache License 2.0');
    expect(links[2]).toHaveAttribute('href', 'https://www.apache.org/licenses/LICENSE-2.0');
  });
});
