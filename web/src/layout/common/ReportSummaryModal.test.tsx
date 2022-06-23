import { fireEvent, render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';

import { Foundation } from '../../types';
import ReportSummaryModal from './ReportSummaryModal';

const mockOnCloseModal = jest.fn();

const defaultProps = {
  foundation: Foundation.cncf,
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

    it('displays loading while image is loading', async () => {
      render(<ReportSummaryModal {...defaultProps} />);

      expect(screen.getByRole('status')).toBeInTheDocument();

      const image = screen.getByAltText('CLOMonitor report summary');
      fireEvent.load(image);

      expect(screen.queryByRole('status')).toBeNull();

      const darkThemeInput = screen.getByRole('radio', { name: 'dark' });
      expect(darkThemeInput).not.toBeChecked();
      await userEvent.click(darkThemeInput);

      expect(screen.getByRole('status')).toBeInTheDocument();

      fireEvent.load(image);

      expect(screen.queryByRole('status')).toBeNull();
    });

    it('renders markdown tab', () => {
      render(<ReportSummaryModal {...defaultProps} />);

      expect(screen.getByText('Embed report summary')).toBeInTheDocument();
      expect(screen.getAllByText('Markdown')).toHaveLength(2);

      expect(screen.getByTestId('code')).toHaveTextContent(
        '[![CLOMonitor report summary](http://localhost/api/projects/cncf/org/proj/report-summary?theme=light)](http://localhost/projects/cncf/org/proj)'
      );
      expect(
        screen.getByRole('button', { name: 'Copy report summary markdown link to clipboard' })
      ).toBeInTheDocument();
    });

    it('renders ascii tab', async () => {
      render(<ReportSummaryModal {...defaultProps} />);

      expect(screen.getAllByText('AsciiDoc')).toHaveLength(2);
      const btn = screen.getByRole('button', { name: 'Open tab ascii' });
      expect(btn).toHaveTextContent('AsciiDoc');
      await userEvent.click(btn);

      expect(screen.getByTestId('code')).toHaveTextContent(
        'http://localhost/projects/cncf/org/proj[image:http://localhost/api/projects/cncf/org/proj/report-summary?theme=light[CLOMonitor report summary]]'
      );
      expect(screen.getByRole('button', { name: 'Copy report summary Ascii link to clipboard' })).toBeInTheDocument();
    });

    it('renders html tab', async () => {
      render(<ReportSummaryModal {...defaultProps} />);

      expect(screen.getAllByText('HTML')).toHaveLength(2);
      const btn = screen.getByRole('button', { name: 'Open tab html' });
      expect(btn).toHaveTextContent('HTML');
      await userEvent.click(btn);

      expect(screen.getByTestId('code')).toHaveTextContent(
        '<a href="http://localhost/projects/cncf/org/proj" rel="noopener noreferrer" target="_blank"><img src="http://localhost/api/projects/cncf/org/proj/report-summary?theme=light" alt="CLOMonitor report summary" /></a>'
      );
      expect(screen.getByRole('button', { name: 'Copy report summary html link to clipboard' })).toBeInTheDocument();
    });
  });
});
