import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { BrowserRouter as Router } from 'react-router-dom';

import SampleQueries from './SampleQueries';

const mockQueries = [
  {
    name: 'Only graduated projects',
    filters: {
      pageNumber: 1,
      filters: { maturity: ['graduated'] },
    },
  },
  {
    name: 'Only incubating projects',
    filters: {
      pageNumber: 1,
      filters: { maturity: ['incubating'] },
    },
  },
  {
    name: 'Only sandbox projects',
    filters: {
      pageNumber: 1,
      filters: { maturity: ['sandbox'] },
    },
  },
];

jest.mock('lodash', () => ({
  ...(jest.requireActual('lodash') as {}),
  sampleSize: () => {
    return mockQueries;
  },
}));

describe('SampleQueries', () => {
  afterEach(() => {
    jest.resetAllMocks();
  });

  it('creates snapshot', () => {
    const { asFragment } = render(
      <Router>
        <SampleQueries />
      </Router>
    );

    expect(asFragment()).toMatchSnapshot();
  });

  describe('Render', () => {
    it('renders component', () => {
      render(
        <Router>
          <SampleQueries />
        </Router>
      );

      expect(screen.getAllByRole('link', { name: /Filter by/ })).toHaveLength(mockQueries.length);

      for (let i = 0; i < mockQueries.length; i++) {
        expect(screen.getByText(mockQueries[i].name)).toBeInTheDocument();
      }
    });

    it('renders proper classes', () => {
      render(
        <Router>
          <SampleQueries className="badge-light border-secondary text-secondary" />
        </Router>
      );

      const links = screen.getAllByRole('link', { name: /Filter by/ });
      expect(links[0]).toHaveClass('badge-light border-secondary text-secondary');
    });

    it('renders break line', () => {
      render(
        <Router>
          <SampleQueries lineBreakIn={2} />
        </Router>
      );

      expect(screen.getByTestId('sampleQueryBreakLine')).toBeInTheDocument();
    });

    it('opens first sample query', () => {
      render(
        <Router>
          <SampleQueries />
        </Router>
      );

      const links = screen.getAllByRole('link', { name: /Filter by/ });
      userEvent.click(links[0]);

      expect(window.location.pathname).toBe('/search');
      expect(window.location.search).toBe('?maturity=graduated&page=1');
    });
  });
});
