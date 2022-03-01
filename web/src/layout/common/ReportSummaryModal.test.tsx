import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';

import ReportSummaryModal from './ReportSummaryModal';

const mockOnCloseModal = jest.fn();

const defaultProps = {
  orgName: 'org',
  projectName: 'proj',
  openStatus: { status: true, name: 'reportSummary' },
  onCloseModal: mockOnCloseModal,
};

describe('ReportSummaryModal', () => {
  afterEach(() => {
    jest.resetAllMocks();
  });

  it('creates snapshot', () => {
    const { asFragment } = render(<ReportSummaryModal {...defaultProps} />);
    expect(asFragment()).toMatchSnapshot();
  });

  describe('Render', () => {
    it('renders proper content', () => {
      render(<ReportSummaryModal {...defaultProps} />);

      expect(screen.getByText('Embed report summary')).toBeInTheDocument();
      expect(screen.getByText('Theme')).toBeInTheDocument();
      expect(screen.getByRole('radio', { name: 'light' })).toBeInTheDocument();
      expect(screen.getByRole('radio', { name: 'light' })).toBeChecked();
      expect(screen.getByRole('radio', { name: 'dark' })).toBeInTheDocument();
      expect(screen.getByRole('button', { name: 'Open tab markdown' })).toBeInTheDocument();
      expect(screen.getByRole('button', { name: 'Open tab ascii' })).toBeInTheDocument();
      expect(screen.getByRole('button', { name: 'Open tab html' })).toBeInTheDocument();
      expect(screen.getByText('Preview')).toBeInTheDocument();
      expect(screen.getByAltText('CLOMonitor report summary')).toBeInTheDocument();
    });

    it('renders markdown tab', () => {
      render(<ReportSummaryModal {...defaultProps} />);

      expect(screen.getByText('Embed report summary')).toBeInTheDocument();
      expect(screen.getAllByText('Markdown')).toHaveLength(2);

      expect(screen.getByTestId('code')).toHaveTextContent(
        '[![CLOMonitor report summary](http://localhost/api/projects/org/proj/report-summary?theme=light)](http://localhost/projects/org/proj)'
      );
      expect(
        screen.getByRole('button', { name: 'Copy report summary markdown link to clipboard' })
      ).toBeInTheDocument();
    });

    it('renders ascii tab', () => {
      render(<ReportSummaryModal {...defaultProps} />);

      expect(screen.getAllByText('AsciiDoc')).toHaveLength(2);
      const btn = screen.getByRole('button', { name: 'Open tab ascii' });
      expect(btn).toHaveTextContent('AsciiDoc');
      userEvent.click(btn);

      expect(screen.getByTestId('code')).toHaveTextContent(
        'http://localhost/projects/org/proj[image:http://localhost/api/projects/org/proj/report-summary?theme=light[CLOMonitor report summary]]'
      );
      expect(screen.getByRole('button', { name: 'Copy report summary Ascii link to clipboard' })).toBeInTheDocument();
    });

    it('renders html tab', () => {
      render(<ReportSummaryModal {...defaultProps} />);

      expect(screen.getAllByText('HTML')).toHaveLength(2);
      const btn = screen.getByRole('button', { name: 'Open tab html' });
      expect(btn).toHaveTextContent('HTML');
      userEvent.click(btn);

      expect(screen.getByTestId('code')).toHaveTextContent(
        '<a href="http://localhost/projects/org/proj" rel="noopener noreferrer" target="_blank"><img src="http://localhost/api/projects/org/proj/report-summary?theme=light" alt="CLOMonitor report summary" /></a>'
      );
      expect(screen.getByRole('button', { name: 'Copy report summary html link to clipboard' })).toBeInTheDocument();
    });
  });
});
