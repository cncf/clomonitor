import { render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { mocked } from 'jest-mock';
import ReactRouter, { BrowserRouter as Router } from 'react-router-dom';

import API from '../../api';
import { ProjectDetail } from '../../types';
import Detail from './index';
jest.mock('../../api');

const mockUseNavigate = jest.fn();

const getMockDetail = (fixtureId: string): ProjectDetail => {
  return require(`./__fixtures__/index/${fixtureId}.json`) as ProjectDetail;
};

let path = {
  pathname: '/projects/artifact-hub/artifact-hub',
  search: '',
  hash: '',
  state: { currentSearch: '?maturity=2&rating=a&page=1' },
  key: 'key',
};

jest.mock('react-router-dom', () => ({
  ...(jest.requireActual('react-router-dom') as any),
  useParams: jest.fn(),
  useLocation: jest.fn(),
  useNavigate: () => mockUseNavigate,
}));

jest.mock('moment', () => ({
  ...(jest.requireActual('moment') as {}),
  unix: () => ({
    fromNow: () => '3 days ago',
  }),
}));

describe('Project detail index', () => {
  beforeEach(() => {
    jest.spyOn(ReactRouter, 'useParams').mockReturnValue({ org: 'org', project: 'proj' });
    jest.spyOn(ReactRouter, 'useLocation').mockReturnValue(path);
  });

  afterEach(() => {
    jest.resetAllMocks();
  });

  it('creates snapshot', async () => {
    const mockProject = getMockDetail('1');
    mocked(API).getProjectDetail.mockResolvedValue(mockProject);

    const { asFragment } = render(
      <Router>
        <Detail />
      </Router>
    );

    await waitFor(() => {
      expect(API.getProjectDetail).toHaveBeenCalledTimes(1);
      expect(asFragment()).toMatchSnapshot();
    });
  });

  describe('Render', () => {
    it('renders component', async () => {
      const mockProject = getMockDetail('1');
      mocked(API).getProjectDetail.mockResolvedValue(mockProject);

      render(
        <Router>
          <Detail />
        </Router>
      );

      await waitFor(() => {
        expect(API.getProjectDetail).toHaveBeenCalledTimes(1);
        expect(API.getProjectDetail).toHaveBeenCalledWith('org', 'proj');
      });

      expect(screen.getByAltText('Artifact Hub logo')).toBeInTheDocument();
      expect(screen.getAllByText('Artifact Hub')).toHaveLength(2);
      expect(
        screen.getByText(
          'Artifact Hub is a web-based application that enables finding, installing, and publishing packages and configurations for CNCF projects.'
        )
      ).toBeInTheDocument();
      expect(screen.getByText('Sandbox')).toBeInTheDocument();
      expect(screen.getByText('App definition')).toBeInTheDocument();
      expect(screen.getAllByText('artifact-hub')).toHaveLength(2);
      expect(screen.getByRole('link', { name: 'Repository link' })).toBeInTheDocument();
    });

    it('renders Back to results', async () => {
      jest.spyOn(ReactRouter, 'useLocation').mockReturnValue({
        ...path,
        state: { currentSearch: '?maturity=2&rating=a&page=1' },
      });

      const mockProject = getMockDetail('1');
      mocked(API).getProjectDetail.mockResolvedValue(mockProject);

      render(
        <Router>
          <Detail />
        </Router>
      );

      await waitFor(() => {
        expect(API.getProjectDetail).toHaveBeenCalledTimes(1);
        expect(API.getProjectDetail).toHaveBeenCalledWith('org', 'proj');
      });

      const backBtn = screen.getByRole('button', { name: 'Back to results' });
      expect(backBtn).toBeInTheDocument();

      userEvent.click(backBtn);

      expect(mockUseNavigate).toHaveBeenCalledTimes(1);
      expect(mockUseNavigate).toHaveBeenCalledWith('/search?maturity=2&rating=a&page=1');
    });

    it('renders placeholder when no data', async () => {
      mocked(API).getProjectDetail.mockRejectedValue('');

      render(
        <Router>
          <Detail />
        </Router>
      );

      await waitFor(() => {
        expect(API.getProjectDetail).toHaveBeenCalledTimes(1);
        expect(API.getProjectDetail).toHaveBeenCalledWith('org', 'proj');
      });

      expect(screen.getByText('Sorry, the project you requested was not found.')).toBeInTheDocument();
      expect(screen.getByText('The project you are looking for may have been deleted.')).toBeInTheDocument();
    });
  });
});
