import { render, screen } from '@testing-library/react';

import { AppContext } from '../../context/AppContextProvider';
import { SortBy, SortDirection } from '../../types';
import Image from './Image';

const defaultProps = {
  url: 'http://img.url',
  dark_url: 'http://dark-img.url',
  alt: 'image',
};

const mockCtx = {
  prefs: {
    search: { limit: 20, sort: { by: SortBy.Name, direction: SortDirection.ASC } },
    theme: { effective: 'light', configured: 'light' },
  },
};

const mockDarkCtx = {
  prefs: {
    search: { limit: 20, sort: { by: SortBy.Name, direction: SortDirection.ASC } },
    theme: { effective: 'dark', configured: 'dark' },
  },
};

describe('Image', () => {
  afterEach(() => {
    jest.resetAllMocks();
  });

  it('creates snapshot', () => {
    const { asFragment } = render(
      <AppContext.Provider value={{ ctx: mockCtx, dispatch: jest.fn() }}>
        <Image {...defaultProps} />
      </AppContext.Provider>
    );
    expect(asFragment()).toMatchSnapshot();
  });

  it('renders proper content', () => {
    render(
      <AppContext.Provider value={{ ctx: mockCtx, dispatch: jest.fn() }}>
        <Image {...defaultProps} />
      </AppContext.Provider>
    );

    const img = screen.getByAltText('image');
    expect(img).toBeInTheDocument();
    expect(img).toHaveAttribute('src', 'http://img.url');
  });

  it('renders dark image', () => {
    render(
      <AppContext.Provider value={{ ctx: mockDarkCtx, dispatch: jest.fn() }}>
        <Image {...defaultProps} />
      </AppContext.Provider>
    );

    const img = screen.getByAltText('image');
    expect(img).toBeInTheDocument();
    expect(img).toHaveAttribute('src', 'http://dark-img.url');
  });

  it('renders placeholder', () => {
    render(
      <AppContext.Provider value={{ ctx: mockCtx, dispatch: jest.fn() }}>
        <Image {...defaultProps} url={null} />
      </AppContext.Provider>
    );

    expect(screen.queryByAltText('image')).toBeNull();
    expect(screen.getByTestId('img-placeholder')).toBeInTheDocument();
  });
});
