import { act, render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';

import DropdownOnHover from './DropdownOnHover';

describe('DropdownOnHover', () => {
  afterEach(() => {
    jest.resetAllMocks();
  });

  it('creates snapshot', () => {
    const { asFragment } = render(
      <DropdownOnHover linkContent="content">
        <>children</>
      </DropdownOnHover>
    );
    expect(asFragment()).toMatchSnapshot();
  });

  describe('Render', () => {
    it('renders component', () => {
      render(
        <DropdownOnHover linkContent="content">
          <>children</>
        </DropdownOnHover>
      );

      expect(screen.getByText('content')).toBeInTheDocument();
      expect(screen.getByRole('complementary')).toBeInTheDocument();
    });

    it('displays dropdown to enter on content and hides on leave', async () => {
      jest.useFakeTimers();

      render(
        <DropdownOnHover linkContent="content">
          <>children</>
        </DropdownOnHover>
      );

      const dropdown = screen.getByRole('complementary');

      expect(dropdown).not.toHaveClass('show');

      userEvent.hover(screen.getByText('content'));

      act(() => {
        jest.advanceTimersByTime(100);
      });

      expect(dropdown).toHaveClass('show');
      expect(screen.getByRole('complementary')).toHaveTextContent('children');

      userEvent.unhover(screen.getByText('content'));

      act(() => {
        jest.advanceTimersByTime(50);
      });

      expect(dropdown).not.toHaveClass('show');

      jest.useRealTimers();
    });

    it('renders correct styles when tooltipStyle is enabled', async () => {
      jest.useFakeTimers();

      render(
        <DropdownOnHover linkContent="content" tooltipStyle>
          <>children</>
        </DropdownOnHover>
      );

      const dropdown = screen.getByRole('complementary');
      expect(dropdown).not.toHaveClass('show');

      userEvent.hover(screen.getByText('content'));

      act(() => {
        jest.advanceTimersByTime(100);
      });

      expect(dropdown).toHaveClass('show tooltipDropdown');
      expect(screen.getByRole('complementary')).toHaveTextContent('children');
      expect(screen.getByTestId('dropdown-arrow')).toBeInTheDocument();

      userEvent.unhover(screen.getByText('content'));

      act(() => {
        jest.advanceTimersByTime(50);
      });

      expect(dropdown).not.toHaveClass('show');

      jest.useRealTimers();
    });

    it('hides dropdown to leave it', async () => {
      jest.useFakeTimers();

      render(
        <DropdownOnHover linkContent="content">
          <>children</>
        </DropdownOnHover>
      );

      const dropdown = screen.getByRole('complementary');

      userEvent.hover(screen.getByText('content'));
      userEvent.hover(dropdown);
      userEvent.unhover(screen.getByText('content'));

      act(() => {
        jest.advanceTimersByTime(100);
      });

      expect(dropdown).toHaveClass('show');

      userEvent.unhover(dropdown);

      act(() => {
        jest.advanceTimersByTime(50);
      });

      expect(dropdown).not.toHaveClass('show');

      jest.useRealTimers();
    });
  });
});
