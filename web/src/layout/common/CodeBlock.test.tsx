import { render, screen } from '@testing-library/react';

import CodeBlock from './CodeBlock';

const defaultProps = {
  language: 'markdown',
  content: '##Sample',
  label: 'Copy btn',
};

describe('CodeBlock', () => {
  afterEach(() => {
    jest.resetAllMocks();
  });

  it('creates snapshot', () => {
    const { asFragment } = render(<CodeBlock {...defaultProps} />);
    expect(asFragment()).toMatchSnapshot();
  });

  it('renders proper content', () => {
    render(<CodeBlock {...defaultProps} />);

    const code = screen.getByTestId('code');
    expect(code).toBeInTheDocument();
    expect(code).toHaveTextContent('##Sample');

    expect(screen.getByRole('button', { name: 'Copy btn' })).toBeInTheDocument();
  });
});
