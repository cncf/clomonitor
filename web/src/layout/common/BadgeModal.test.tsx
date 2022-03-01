import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';

import BadgeModal from './BadgeModal';

const mockOnCloseModal = jest.fn();

const defaultProps = {
  orgName: 'org',
  projectName: 'proj',
  openStatus: { status: true, name: 'badge' },
  onCloseModal: mockOnCloseModal,
};

describe('BadgeModal', () => {
  afterEach(() => {
    jest.resetAllMocks();
  });

  it('creates snapshot', () => {
    const { asFragment } = render(<BadgeModal {...defaultProps} />);
    expect(asFragment()).toMatchSnapshot();
  });

  describe('Render', () => {
    it('renders markdown tab', () => {
      render(<BadgeModal {...defaultProps} />);

      expect(screen.getByText('Project badge')).toBeInTheDocument();
      expect(screen.getAllByText('Markdown')).toHaveLength(2);
      expect(screen.getAllByRole('button', { name: /Open tab/ })).toHaveLength(2);

      const badge = screen.getByAltText('CLOMonitor badge');
      expect(badge).toBeInTheDocument();
      expect(badge).toHaveProperty(
        'src',
        'https://img.shields.io/endpoint?url=http://localhost/api/projects/org/proj/badge'
      );
      const code = screen.getByTestId('code');
      expect(code).toBeInTheDocument();
      expect(code).toHaveTextContent(
        '[![CLOMonitor](https://img.shields.io/endpoint?url=http://localhost/api/projects/org/proj/badge)](http://localhost/projects/org/proj)'
      );
    });

    it('renders ascii tab', () => {
      render(<BadgeModal {...defaultProps} />);

      expect(screen.getAllByText('AsciiDoc')).toHaveLength(2);
      const btns = screen.getAllByRole('button', { name: /Open tab/ });
      expect(btns[1]).toHaveTextContent('AsciiDoc');
      userEvent.click(btns[1]);

      const badge = screen.getByAltText('CLOMonitor badge');
      expect(badge).toBeInTheDocument();
      expect(badge).toHaveProperty(
        'src',
        'https://img.shields.io/endpoint?url=http://localhost/api/projects/org/proj/badge'
      );
      const code = screen.getByTestId('code');
      expect(code).toBeInTheDocument();
      expect(code).toHaveTextContent(
        'http://localhost/projects/org/proj[image:https://img.shields.io/endpoint?url=http://localhost/api/projects/org/proj/badge[CLOMonitor]]'
      );
    });
  });
});
