import { render, screen } from '@testing-library/react';

import Image from './Image';

const defaultProps = {
  url: 'http://img.url',
  alt: 'image',
};

describe('Image', () => {
  afterEach(() => {
    jest.resetAllMocks();
  });

  it('creates snapshot', () => {
    const { asFragment } = render(<Image {...defaultProps} />);
    expect(asFragment()).toMatchSnapshot();
  });

  it('renders proper content', () => {
    render(<Image {...defaultProps} />);

    const img = screen.getByAltText('image');
    expect(img).toBeInTheDocument();
    expect(img).toHaveAttribute('src', 'http://img.url');
  });

  it('renders placeholder', () => {
    render(<Image {...defaultProps} url={null} />);

    expect(screen.queryByAltText('image')).toBeNull();
    expect(screen.getByTestId('img-placeholder')).toBeInTheDocument();
  });
});
