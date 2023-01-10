import { render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';

import Timeline from './Timeline';

const mockSetActiveDate = jest.fn();

const defaultProps = {
  snapshots: ['2022-10-28', '2022-10-27', '2022-10-26', '2022-09-10', '2022-08-07', '2021-04-18', '2020-03-09'],
  setActiveDate: mockSetActiveDate,
};

describe('Timeline', () => {
  let dateNowSpy: any;

  beforeEach(() => {
    dateNowSpy = jest.spyOn(Date, 'now').mockImplementation(() => 1667029023000);
  });

  afterAll(() => {
    dateNowSpy.mockRestore();
  });

  afterEach(() => {
    jest.resetAllMocks();
  });

  it('creates snapshot', () => {
    const { asFragment } = render(<Timeline {...defaultProps} />);

    expect(asFragment()).toMatchSnapshot();
  });

  describe('Render', () => {
    it('renders timeline', async () => {
      render(<Timeline {...defaultProps} />);

      const buttons = screen.getAllByRole('button', { name: /Opens snapshot:/ });
      expect(buttons).toHaveLength(8);
      expect(buttons[0]).toHaveTextContent('29');
      expect(buttons[1]).toHaveTextContent('28');
      expect(buttons[2]).toHaveTextContent('27');
      expect(buttons[3]).toHaveTextContent('26');
      expect(buttons[4]).toHaveTextContent('10');
      expect(buttons[5]).toHaveTextContent('7');
      expect(buttons[6]).toHaveTextContent('18');
      expect(buttons[7]).toHaveTextContent('9');

      expect(screen.getByText('2022')).toBeInTheDocument();
      expect(screen.getByText('2021')).toBeInTheDocument();
      expect(screen.getByText('2020')).toBeInTheDocument();
      expect(screen.getByText('Oct')).toBeInTheDocument();
      expect(screen.getByText('Sep')).toBeInTheDocument();
      expect(screen.getByText('Aug')).toBeInTheDocument();
      expect(screen.getByText('Apr')).toBeInTheDocument();
      expect(screen.getByText('Mar')).toBeInTheDocument();
    });

    it('clicks day button', async () => {
      const { rerender } = render(<Timeline {...defaultProps} />);

      const buttons = screen.getAllByRole('button', { name: /Opens snapshot:/ });
      expect(buttons[0]).toHaveClass('activeDot');

      await userEvent.click(buttons[3]);

      await waitFor(() => {
        expect(mockSetActiveDate).toHaveBeenCalledTimes(1);
        expect(mockSetActiveDate).toHaveBeenCalledWith('2022-10-26');

        rerender(<Timeline {...defaultProps} activeDate="2022-10-26" />);

        expect(buttons[3]).toHaveClass('activeDot');
      });
    });

    it('clicks Now day button', async () => {
      render(<Timeline {...defaultProps} activeDate="2022-10-26" />);

      const buttons = screen.getAllByRole('button', { name: /Opens snapshot:/ });
      expect(buttons[3]).toHaveClass('activeDot');

      await userEvent.click(buttons[0]);

      await waitFor(() => {
        expect(mockSetActiveDate).toHaveBeenCalledTimes(1);
        expect(mockSetActiveDate).toHaveBeenCalledWith(undefined);
      });
    });

    describe('does not render', () => {
      it('when snapshots list is empty', () => {
        const { container } = render(<Timeline {...defaultProps} snapshots={[]} />);

        expect(container).toBeEmptyDOMElement();
      });

      it('when snapshots list + today length is less than 3 items', () => {
        const { container } = render(<Timeline {...defaultProps} snapshots={['2022-10-28']} />);

        expect(container).toBeEmptyDOMElement();
      });
    });
  });
});
