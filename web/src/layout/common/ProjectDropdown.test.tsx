import { fireEvent, render, screen } from '@testing-library/react';

import { Foundation } from '../../types';
import ProjectDropdown from './ProjectDropdown';

const defaultProps = {
  foundation: Foundation.cncf,
  orgName: 'org',
  projectName: 'proj',
};

describe('ProjectDropdown', () => {
  afterEach(() => {
    jest.resetAllMocks();
  });

  it('creates snapshot', () => {
    const { asFragment } = render(<ProjectDropdown {...defaultProps} />);
    expect(asFragment()).toMatchSnapshot();
  });

  it('renders proper content', () => {
    render(<ProjectDropdown {...defaultProps} />);

    const dropdown = screen.getByRole('complementary');
    expect(dropdown).not.toHaveClass('show');

    const btn = screen.getByTestId('dropdown-btn');
    expect(btn).toBeInTheDocument();

    fireEvent.click(btn);

    expect(dropdown).toHaveClass('show');
    expect(screen.getByRole('button', { name: 'Get badge' })).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Embed report summary' })).toBeInTheDocument();
  });

  it('opens Get badge modal', () => {
    render(<ProjectDropdown {...defaultProps} />);

    const dropdown = screen.getByRole('complementary');
    expect(dropdown).not.toHaveClass('show');

    fireEvent.click(screen.getByTestId('dropdown-btn'));

    expect(dropdown).toHaveClass('show');
    const badgeBtn = screen.getByRole('button', { name: 'Get badge' });
    fireEvent.click(badgeBtn);

    expect(dropdown).not.toHaveClass('show');
    expect(screen.getByText('Project badge')).toBeInTheDocument();
  });

  it('opens Report summary modal', () => {
    render(<ProjectDropdown {...defaultProps} />);

    const dropdown = screen.getByRole('complementary');
    expect(dropdown).not.toHaveClass('show');

    fireEvent.click(screen.getByTestId('dropdown-btn'));

    expect(dropdown).toHaveClass('show');
    const reportBtn = screen.getByRole('button', { name: 'Embed report summary' });
    fireEvent.click(reportBtn);

    expect(dropdown).not.toHaveClass('show');
    expect(screen.getAllByText('Embed report summary')).toHaveLength(2);
  });
});
