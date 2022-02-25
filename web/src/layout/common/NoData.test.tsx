import { render, screen } from '@testing-library/react';

import NoData from './NoData';

describe('NoData', () => {
  afterEach(() => {
    jest.resetAllMocks();
  });

  it('creates snapshot', () => {
    const { asFragment } = render(
      <NoData>
        <>no data</>
      </NoData>
    );
    expect(asFragment()).toMatchSnapshot();
  });

  it('renders proper content', () => {
    render(
      <NoData>
        <>no data</>
      </NoData>
    );

    expect(screen.getByRole('alert')).toBeInTheDocument();
    expect(screen.getByRole('alert')).toHaveTextContent('no data');
  });
});
