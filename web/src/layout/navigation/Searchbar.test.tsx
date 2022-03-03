import { render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { mocked } from 'jest-mock';
import ReactRouter, { BrowserRouter as Router } from 'react-router-dom';

import API from '../../api';
import { Project } from '../../types';
import prepareQueryString from '../../utils/prepareQueryString';
import Searchbar from './Searchbar';
jest.mock('../../api');

const mockUseNavigate = jest.fn();

jest.mock('react-router-dom', () => ({
  ...(jest.requireActual('react-router-dom') as any),
  useNavigate: () => mockUseNavigate,
  useSearchParams: jest.fn(),
}));

interface SearchResults {
  items: Project[];
  'Pagination-Total-Count': string;
}

const getMockSearch = (fixtureId: string): SearchResults => {
  return require(`./__fixtures__/Searchbar/${fixtureId}.json`) as SearchResults;
};

const mockSetScrollPosition = jest.fn();

const defaultProps = {
  setScrollPosition: mockSetScrollPosition,
};

describe('Searchbar', () => {
  beforeEach(() => {
    jest.spyOn(ReactRouter, 'useSearchParams').mockImplementation(() => [new URLSearchParams(''), jest.fn()]);
  });

  afterEach(() => {
    jest.resetAllMocks();
  });

  it('creates snapshot', () => {
    const { asFragment } = render(
      <Router>
        <Searchbar {...defaultProps} />
      </Router>
    );
    expect(asFragment()).toMatchSnapshot();
  });

  it('renders proper content', () => {
    render(
      <Router>
        <Searchbar {...defaultProps} />
      </Router>
    );

    expect(screen.getByPlaceholderText('Search projects')).toBeInTheDocument();
    expect(screen.queryByRole('button', { name: 'Clear search' })).toBeNull();
    expect(screen.getByRole('button', { name: 'Search text' })).toBeInTheDocument();
  });

  it('renders with text', () => {
    jest.spyOn(ReactRouter, 'useSearchParams').mockImplementation(() => [new URLSearchParams('?text=test'), jest.fn()]);

    render(
      <Router>
        <Searchbar {...defaultProps} />
      </Router>
    );

    expect(screen.getByRole('textbox')).toBeInTheDocument();
    expect(screen.getByRole('textbox')).toHaveValue('test');
    expect(screen.getByRole('button', { name: 'Clear search' })).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Search text' })).toBeInTheDocument();
  });

  describe('clear btn', () => {
    it('clear input', () => {
      jest
        .spyOn(ReactRouter, 'useSearchParams')
        .mockImplementation(() => [new URLSearchParams(''), jest.fn()])
        .mockImplementationOnce(() => [new URLSearchParams('?text=test'), jest.fn()])
        .mockImplementationOnce(() => [new URLSearchParams('?text=test'), jest.fn()])
        .mockImplementationOnce(() => [new URLSearchParams('?text=test'), jest.fn()])
        .mockImplementationOnce(() => [new URLSearchParams('?text=testing'), jest.fn()])
        .mockImplementationOnce(() => [new URLSearchParams('?text=testing'), jest.fn()]);

      render(
        <Router>
          <Searchbar {...defaultProps} />
        </Router>
      );

      const clearBtn = screen.getByRole('button', { name: 'Clear search' });
      const input = screen.getByRole('textbox');

      expect(input).toBeInTheDocument();
      expect(input).toHaveValue('test');
      userEvent.type(input, 'ing');
      userEvent.click(clearBtn);
      expect(input).toHaveValue('');
    });
  });

  it('updates value on change input', async () => {
    jest
      .spyOn(ReactRouter, 'useSearchParams')
      .mockImplementation(() => [new URLSearchParams('?text=testing'), jest.fn()])
      .mockImplementationOnce(() => [new URLSearchParams(''), jest.fn()]);

    const mockSearch = getMockSearch('1');
    mocked(API).searchProjects.mockResolvedValue(mockSearch);

    render(
      <Router>
        <Searchbar {...defaultProps} />
      </Router>
    );

    const input = screen.getByRole('textbox');

    userEvent.type(input, 'testing');

    expect(input).toHaveValue('testing');

    await waitFor(() => {
      expect(API.searchProjects).toHaveBeenCalledTimes(1);
    });
  });

  describe('search projects', () => {
    it('display search results', async () => {
      jest
        .spyOn(ReactRouter, 'useSearchParams')
        .mockImplementation(() => [new URLSearchParams('?text=testing'), jest.fn()])
        .mockImplementationOnce(() => [new URLSearchParams(''), jest.fn()]);

      const mockSearch = getMockSearch('1');
      mocked(API).searchProjects.mockResolvedValue(mockSearch);

      render(
        <Router>
          <Searchbar {...defaultProps} />
        </Router>
      );

      const input = screen.getByRole('textbox') as HTMLInputElement;

      input.focus();

      userEvent.type(input, 'testing');
      expect(input.value).toBe('testing');

      await waitFor(() => {
        expect(API.searchProjects).toHaveBeenCalledTimes(1);
        expect(API.searchProjects).toHaveBeenCalledWith({
          limit: 5,
          offset: 0,
          sort_by: 'name',
          sort_direction: 'asc',
          text: 'testing',
        });
      });

      expect(screen.getByRole('listbox')).toBeInTheDocument();
      expect(screen.getAllByRole('option')).toHaveLength(8);
    });

    it("doesn't display results when input is not focused", async () => {
      const useSearchParamsSpy = jest.spyOn(ReactRouter, 'useSearchParams');
      useSearchParamsSpy.mockImplementation(() => [new URLSearchParams('?text=test'), jest.fn()]);

      const mockSearch = getMockSearch('1');
      mocked(API).searchProjects.mockResolvedValue(mockSearch);

      render(
        <Router>
          <Searchbar {...defaultProps} />
        </Router>
      );

      const input = screen.getByDisplayValue('test');

      input.focus();
      userEvent.type(input, 'ing');
      input.blur();

      await waitFor(() => {
        expect(API.searchProjects).toHaveBeenCalledTimes(1);
        input.blur();
      });

      expect(screen.queryByRole('listbox')).toBeNull();
    });

    it('loads project detail from search dropdown', async () => {
      jest
        .spyOn(ReactRouter, 'useSearchParams')
        .mockImplementation(() => [new URLSearchParams('?text=testing'), jest.fn()])
        .mockImplementationOnce(() => [new URLSearchParams(''), jest.fn()]);

      const mockSearch = getMockSearch('1');
      mocked(API).searchProjects.mockResolvedValue(mockSearch);

      render(
        <Router>
          <Searchbar {...defaultProps} />
        </Router>
      );

      const input = screen.getByRole('textbox') as HTMLInputElement;

      input.focus();

      userEvent.type(input, 'testing');
      expect(input.value).toBe('testing');

      await waitFor(() => {
        expect(API.searchProjects).toHaveBeenCalledTimes(1);
        expect(API.searchProjects).toHaveBeenCalledWith({
          limit: 5,
          offset: 0,
          sort_by: 'name',
          sort_direction: 'asc',
          text: 'testing',
        });
      });

      expect(screen.getByRole('listbox')).toBeInTheDocument();
      const items = screen.getAllByRole('option');
      userEvent.click(items[1]);

      expect(mockUseNavigate).toHaveBeenCalledTimes(1);
      expect(mockUseNavigate).toHaveBeenCalledWith('/projects/backstage/backstage');
    });

    it('loads new search from search dropdown', async () => {
      jest
        .spyOn(ReactRouter, 'useSearchParams')
        .mockImplementation(() => [new URLSearchParams('?text=testing'), jest.fn()])
        .mockImplementationOnce(() => [new URLSearchParams(''), jest.fn()]);

      const mockSearch = getMockSearch('1');
      mocked(API).searchProjects.mockResolvedValue(mockSearch);

      render(
        <Router>
          <Searchbar {...defaultProps} />
        </Router>
      );

      const input = screen.getByRole('textbox') as HTMLInputElement;

      input.focus();

      userEvent.type(input, 'testing');
      expect(input.value).toBe('testing');

      await waitFor(() => {
        expect(API.searchProjects).toHaveBeenCalledTimes(1);
        expect(API.searchProjects).toHaveBeenCalledWith({
          limit: 5,
          offset: 0,
          sort_by: 'name',
          sort_direction: 'asc',
          text: 'testing',
        });
      });

      expect(screen.getByRole('listbox')).toBeInTheDocument();
      const allResults = screen.getByRole('option', { name: 'See all results' });
      userEvent.click(allResults);

      expect(mockUseNavigate).toHaveBeenCalledTimes(1);
      expect(mockUseNavigate).toHaveBeenCalledWith({ pathname: '/search', search: '?text=testing&page=1' });
    });

    it('uses arrow for seleting one item and loads detail to click enter', async () => {
      jest
        .spyOn(ReactRouter, 'useSearchParams')
        .mockImplementation(() => [new URLSearchParams('?text=testing'), jest.fn()])
        .mockImplementationOnce(() => [new URLSearchParams(''), jest.fn()]);

      const mockSearch = getMockSearch('1');
      mocked(API).searchProjects.mockResolvedValue(mockSearch);

      render(
        <Router>
          <Searchbar {...defaultProps} />
        </Router>
      );

      const input = screen.getByRole('textbox') as HTMLInputElement;

      input.focus();

      userEvent.type(input, 'testing');
      expect(input.value).toBe('testing');

      await waitFor(() => {
        expect(API.searchProjects).toHaveBeenCalledTimes(1);
        expect(API.searchProjects).toHaveBeenCalledWith({
          limit: 5,
          offset: 0,
          sort_by: 'name',
          sort_direction: 'asc',
          text: 'testing',
        });
      });

      expect(screen.getByRole('listbox')).toBeInTheDocument();
      const options = screen.getAllByRole('option');
      expect(options[0]).not.toHaveClass('activeDropdownItem');

      userEvent.keyboard('{arrowdown}');

      await waitFor(() => {
        expect(options[0]).toHaveClass('activeDropdownItem');
      });

      userEvent.keyboard('{arrowdown}{arrowdown}');

      expect(options[0]).not.toHaveClass('activeDropdownItem');
      expect(options[2]).toHaveClass('activeDropdownItem');

      userEvent.keyboard('{enter}');

      expect(mockUseNavigate).toHaveBeenCalledTimes(1);
      expect(mockUseNavigate).toHaveBeenCalledWith('/projects/keptn/keptn');
    });
  });

  describe('Navigate', () => {
    it('calls on Enter key press', () => {
      jest
        .spyOn(ReactRouter, 'useSearchParams')
        .mockImplementation(() => [new URLSearchParams('?text=testing'), jest.fn()])
        .mockImplementationOnce(() => [new URLSearchParams(''), jest.fn()]);

      render(
        <Router>
          <Searchbar {...defaultProps} />
        </Router>
      );

      const input = screen.getByRole('textbox');
      userEvent.type(input, 'testing{enter}');
      expect(input).not.toBe(document.activeElement);
      expect(mockUseNavigate).toHaveBeenCalledTimes(1);
      expect(mockUseNavigate).toHaveBeenCalledWith({
        pathname: '/search',
        search: prepareQueryString({
          text: 'testing',
          pageNumber: 1,
        }),
      });
    });

    it('calls navigate on Enter key press when text is empty with undefined text', () => {
      render(
        <Router>
          <Searchbar {...defaultProps} />
        </Router>
      );

      const input = screen.getByPlaceholderText('Search projects');
      userEvent.type(input, '{enter}');
      expect(mockUseNavigate).toHaveBeenCalledTimes(1);
      expect(mockUseNavigate).toHaveBeenCalledWith({
        pathname: '/search',
        search: prepareQueryString({
          text: undefined,
          pageNumber: 1,
        }),
      });
    });
  });
});
