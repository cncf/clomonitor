import fetchMock, { enableFetchMocks } from 'jest-fetch-mock';

import { ErrorKind, Project, ProjectDetail, SortBy, SortDirection } from '../types';
import API from './index';
enableFetchMocks();

const getData = (fixtureId: string): any => {
  return require(`./__fixtures__/index/${fixtureId}.json`) as any;
};

describe('API', () => {
  afterEach(() => {
    jest.resetAllMocks();
  });

  describe('API', () => {
    beforeEach(() => {
      fetchMock.resetMocks();
    });

    describe('handleErrors', () => {
      it('Other', async () => {
        fetchMock.mockResponse(JSON.stringify({ message: '' }), {
          status: 400,
        });

        await expect(API.getProjectDetail('org1', 'proj1')).rejects.toEqual({
          kind: ErrorKind.Other,
        });
        expect(fetchMock).toHaveBeenCalledTimes(1);
      });

      it('Other with custom message', async () => {
        fetchMock.mockResponse(JSON.stringify({ message: 'custom error' }), {
          headers: {
            'content-type': 'application/json',
          },
          status: 400,
        });

        await expect(API.getProjectDetail('org1', 'proj1')).rejects.toEqual({
          kind: ErrorKind.Other,
          message: 'custom error',
        });
        expect(fetchMock).toHaveBeenCalledTimes(1);
      });
    });

    describe('getProjectDetail', () => {
      it('success', async () => {
        const projectDetail = getData('1') as ProjectDetail;
        fetchMock.mockResponse(JSON.stringify(projectDetail), {
          headers: {
            'content-type': 'application/json',
          },
          status: 200,
        });

        const response = await API.getProjectDetail('org1', 'proj1');

        expect(fetchMock).toHaveBeenCalledTimes(1);
        expect(fetchMock.mock.calls[0][0]).toEqual('/api/projects/org1/proj1');
        expect(response).toEqual(projectDetail);
      });
    });

    describe('searchProjects', () => {
      it('success', async () => {
        const data = getData('2') as Project[];
        fetchMock.mockResponse(JSON.stringify(data), {
          headers: {
            'content-type': 'application/json',
            'Pagination-Total-Count': '7',
          },
          status: 200,
        });

        const response = await API.searchProjects({
          limit: 20,
          offset: 0,
          sort_by: SortBy.Name,
          sort_direction: SortDirection.ASC,
          filters: {
            category: ['0'],
            maturity: ['2'],
          },
        });

        expect(fetchMock).toHaveBeenCalledTimes(1);
        expect(fetchMock.mock.calls[0][0]).toEqual('/api/projects/search');
        expect(response).toEqual(data);
      });
    });
  });
});
