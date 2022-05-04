import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { BrowserRouter as Router } from 'react-router-dom';

import { Repository } from '../../../types';
import Summary from './Summary';

const mockUseNavigate = jest.fn();

jest.mock('react-router-dom', () => ({
  ...(jest.requireActual('react-router-dom') as any),
  useNavigate: () => mockUseNavigate,
}));

const getRepositories = (fixtureId: string): Repository[] => {
  return require(`./__fixtures__/Summary/${fixtureId}.json`) as Repository[];
};

const mockScrollIntoView = jest.fn();

const defaultProps = {
  scrollIntoView: mockScrollIntoView,
};

describe('Summary', () => {
  afterEach(() => {
    jest.resetAllMocks();
  });

  it('creates snapshot', () => {
    const repositories = getRepositories('1');
    const { asFragment } = render(
      <Router>
        <Summary {...defaultProps} repositories={repositories} />
      </Router>
    );
    expect(asFragment()).toMatchSnapshot();
  });

  describe('Render', () => {
    it('renders component', () => {
      const repositories = getRepositories('1');
      render(
        <Router>
          <Summary {...defaultProps} repositories={repositories} />
        </Router>
      );

      expect(screen.getByText('Repository')).toBeInTheDocument();
      expect(screen.getByText('Global')).toBeInTheDocument();
      expect(screen.getByText('Documentation')).toBeInTheDocument();
      expect(screen.getByText('License')).toBeInTheDocument();
      expect(screen.getByText('Best Practices')).toBeInTheDocument();
      expect(screen.getByText('Security')).toBeInTheDocument();

      expect(screen.getByText('spec')).toBeInTheDocument();
      expect(screen.getByText('sdk-go')).toBeInTheDocument();
      expect(screen.getByText('sdk-javascript')).toBeInTheDocument();
      expect(screen.getByText('sdk-csharp')).toBeInTheDocument();
      expect(screen.getByText('sdk-java')).toBeInTheDocument();
      expect(screen.getByText('sdk-python')).toBeInTheDocument();

      const btns = screen.getAllByRole('button', { name: /Go from summary to section:/i });
      expect(btns).toHaveLength(6);

      expect(screen.getByText('44')).toBeInTheDocument();
      expect(screen.getByText('60')).toBeInTheDocument();
      expect(screen.getByText('80')).toBeInTheDocument();
      expect(screen.getByText('35')).toBeInTheDocument();
      expect(screen.getByText('0')).toBeInTheDocument();

      expect(screen.getAllByText('95')).toHaveLength(2);
      expect(screen.getAllByText('90')).toHaveLength(2);
      expect(screen.getAllByText('100')).toHaveLength(5);
      expect(screen.getAllByText('85')).toHaveLength(3);
      expect(screen.getAllByText('70')).toHaveLength(3);
      expect(screen.getAllByText('n/a')).toHaveLength(16);
    });
  });

  it('renders component when one of the repos failed', () => {
    const repositories = getRepositories('2');
    render(
      <Router>
        <Summary {...defaultProps} repositories={repositories} />
      </Router>
    );

    expect(screen.getByText('Repository')).toBeInTheDocument();
    expect(screen.getByText('Global')).toBeInTheDocument();
    expect(screen.getByText('Documentation')).toBeInTheDocument();
    expect(screen.getByText('License')).toBeInTheDocument();
    expect(screen.getByText('Best Practices')).toBeInTheDocument();
    expect(screen.getByText('Security')).toBeInTheDocument();

    expect(screen.getByText('go-control-panel')).toBeInTheDocument();
    expect(screen.getByText('envoy')).toBeInTheDocument();

    const btns = screen.getAllByRole('button', { name: /Go from summary to section:/i });
    expect(btns).toHaveLength(2);

    expect(screen.getByText('75')).toBeInTheDocument();
    expect(screen.getByText('100')).toBeInTheDocument();

    expect(screen.getAllByText('90')).toHaveLength(2);
    expect(screen.getAllByText('87')).toHaveLength(2);
    expect(screen.getAllByText('n/a')).toHaveLength(6);
  });

  it('renders component', async () => {
    const repositories = getRepositories('1');
    render(
      <Router>
        <Summary {...defaultProps} repositories={repositories} />
      </Router>
    );

    const btn = screen.getAllByRole('button', { name: 'Go from summary to section: spec' });
    await userEvent.click(btn[0]);

    expect(mockUseNavigate).toHaveBeenCalledTimes(1);
    expect(mockUseNavigate).toHaveBeenCalledWith(
      {
        hash: 'spec',
        pathname: '/',
      },
      { state: null }
    );
  });

  it('does not render repository without report', () => {
    const repositories = getRepositories('3');
    render(
      <Router>
        <Summary {...defaultProps} repositories={repositories} />
      </Router>
    );

    expect(screen.queryByText('grpc.io')).toBeNull();
  });

  describe('Does not render', () => {
    it('when repositories is empty', () => {
      const { container } = render(
        <Router>
          <Summary {...defaultProps} repositories={[]} />
        </Router>
      );
      expect(container).toBeEmptyDOMElement();
    });
  });
});
