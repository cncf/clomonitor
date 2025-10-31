import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { vi } from 'vitest';

import { ReportOption } from '../../../../types';
import Checks from './index';

const mockOnChecksChange = vi.fn();
const mockOnChange = vi.fn();

const defaultProps = {
  activePassingChecks: [],
  activeNotPassingChecks: [],
  onChecksChange: mockOnChecksChange,
  onChange: mockOnChange,
};

describe('Checks', () => {
  afterEach(() => {
    vi.resetAllMocks();
  });

  it('creates snapshot', () => {
    const { asFragment } = render(<Checks {...defaultProps} />);

    expect(asFragment()).toMatchSnapshot();
  });

  describe('Render', () => {
    it('renders Checks', () => {
      render(<Checks {...defaultProps} />);

      expect(screen.getByText('Checks')).toBeInTheDocument();
      expect(screen.getByRole('button', { name: 'Open checks modal' })).toBeInTheDocument();
      expect(screen.getByRole('button', { name: 'Open checks modal' })).toHaveTextContent('Add checks filters');
    });

    it('renders Checks with active options', () => {
      render(<Checks {...defaultProps} activePassingChecks={[ReportOption.Adopters]} />);

      expect(screen.getByText('Checks')).toBeInTheDocument();
      expect(screen.getByRole('button', { name: 'Open checks modal' })).toBeInTheDocument();
      expect(screen.getByRole('button', { name: 'Open checks modal' })).toHaveTextContent('Edit checks filters');
    });

    it('renders Checks modal', async () => {
      render(<Checks {...defaultProps} />);

      const btn = screen.getByRole('button', { name: 'Open checks modal' });

      await userEvent.click(btn);

      expect(await screen.findByRole('dialog')).toBeInTheDocument();
      expect(screen.getByText('Checks filters')).toBeInTheDocument();
      expect(screen.getByRole('button', { name: 'Reset checks filters' })).toBeInTheDocument();
      expect(screen.getByRole('button', { name: 'Reset checks filters' })).toBeDisabled();
      expect(screen.getByRole('button', { name: 'Apply filters' })).toBeInTheDocument();
    });

    it('calls onChecksChange to close modal filter', async () => {
      render(<Checks {...defaultProps} />);

      const btn = screen.getByRole('button', { name: 'Open checks modal' });

      await userEvent.click(btn);

      expect(await screen.findByRole('dialog')).toBeInTheDocument();

      const applyBtn = screen.getByRole('button', { name: 'Apply filters' });

      await userEvent.click(applyBtn);

      expect(mockOnChecksChange).toHaveBeenCalledTimes(1);
      expect(mockOnChecksChange).toHaveBeenCalledWith({ not_passing_check: [], passing_check: [] });

      expect(screen.queryByRole('dialog')).toBeNull();
    });

    it('calls onChange to click remove mini button', async () => {
      render(<Checks {...defaultProps} activePassingChecks={[ReportOption.Adopters]} />);

      const btn = screen.getByRole('button', { name: 'Open checks modal' });

      await userEvent.click(btn);

      const dialog = await screen.findByRole('dialog');
      expect(dialog).toBeInTheDocument();

      const removeBtn = (await screen.findAllByLabelText(/Remove passing Adopters check/i))[0];

      await userEvent.click(removeBtn);

      expect(mockOnChange).toHaveBeenCalledTimes(1);
      expect(mockOnChange).toHaveBeenCalledWith('passing_check', 'adopters', false);
    });
  });
});
