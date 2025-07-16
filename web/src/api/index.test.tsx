import fetchMock, { enableFetchMocks } from 'jest-fetch-mock';

import { ErrorKind, Project, ProjectDetail, SortBy, SortDirection } from '../types';
import API from './index';
enableFetchMocks();

const getData = (fixtureId: string): object => {
  // eslint-disable-next-line @typescript-eslint/no-require-imports
  return require(`./__fixtures__/index/${fixtureId}.json`) as object;
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

        await expect(API.getProjectDetail('proj1', 'foundation1')).rejects.toEqual({
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

        await expect(API.getProjectDetail('proj1', 'foundation1')).rejects.toEqual({
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

        const response = await API.getProjectDetail('proj1', 'foundation1');

        expect(fetchMock).toHaveBeenCalledTimes(1);
        expect(fetchMock.mock.calls[0][0]).toEqual('/api/projects/foundation1/proj1');
        expect(response).toEqual(projectDetail);
      });
    });

    describe('searchProjects', () => {
      it('success', async () => {
        const data = getData('2') as Project[];
        fetchMock.mockResponse(JSON.stringify(data), {
          headers: {
            'content-type': 'application/json',
            'Pagination-Total-Count': '4',
          },
          status: 200,
        });

        const response = await API.searchProjects({
          limit: 20,
          offset: 0,
          sort_by: SortBy.Name,
          sort_direction: SortDirection.ASC,
          filters: {
            maturity: ['sandbox', 'incubating'],
          },
        });

        expect(fetchMock).toHaveBeenCalledTimes(1);
        expect(fetchMock.mock.calls[0][0]).toEqual(
          '/api/projects/search?limit=20&offset=0&sort_by=name&sort_direction=asc&maturity[0]=sandbox&maturity[1]=incubating'
        );
        expect(response).toEqual(data);
      });
    });

    describe('getRepositoriesCSV', () => {
      it('success', async () => {
        const csv: string = `
Foundation,Project,Repository URL,Check Sets,Adopters,Changelog,Code of Conduct,Contributing,Governance,Maintainers,Readme,Roadmap,Website,License Approved,License Scanning,License SPDX ID,Analytics,ArtifactHub Badge,CLA,Community Meeting,DCO,GitHub discussions,OpenSSF Badge,Recent Release,Slack Presence,Binary Artifacts,Code Review,Dangerous Workflow,Dependency Update Tool,Maintained,SBOM,Security Policy,Signed Releases,Token Permissions,Trademark Disclaimer
cncf,aeraki-mesh,https://github.com/aeraki-mesh/aeraki,"{community,code}",f,t,t,t,t,t,t,f,f,t,f,Apache-2.0,,f,f,t,t,f,f,t,t,t,f,t,f,t,f,t,f,f,f
cncf,akri,https://github.com/project-akri/akri,"{community,code}",t,t,t,t,t,t,t,t,t,t,f,Apache-2.0,GA4,f,f,t,t,f,t,t,t,t,t,t,f,t,f,t,f,f,f
`;

        fetchMock.mockResponse(csv, {
          headers: {
            'content-type': 'csv',
          },
          status: 200,
        });

        const response = await API.getRepositoriesCSV();

        expect(fetchMock).toHaveBeenCalledTimes(1);
        expect(fetchMock.mock.calls[0][0]).toEqual('/data/repositories.csv');
        expect(response).toEqual(csv);
      });
    });

    describe('getRepositoryReportMD', () => {
      it('success', async () => {
        const reportSample = `
        ## CLOMonitor report

        ### Summary

        **Repository**: artifact-hub
        **URL**: https://github.com/artifacthub/hub
        **Checks sets**:  \`COMMUNITY\` + \`CODE\`
        **Score**: 95

        ### Checks passed per category

        | Category  |  Score  |
        | :----------------- | --------: |
        |  Documentation  |  87%  |
        |  License  |  100%  |
        |  Best Practices  |  95%  |
        |  Security  |  100%  |
        |  Legal  |  100%  |

        ## Checks

        ### Documentation [87%]

          - [x] [Adopters](https://github.com/artifacthub/hub/blob/master/ADOPTERS.md) ([_docs_](https://clomonitor.io/docs/topics/checks/#adopters))
          - [x] Changelog ([_docs_](https://clomonitor.io/docs/topics/checks/#changelog))
          - [x] [Code of conduct](https://github.com/artifacthub/hub/blob/master/code-of-conduct.md) ([_docs_](https://clomonitor.io/docs/topics/checks/#code-of-conduct))
          - [x] [Contributing](https://github.com/artifacthub/hub/blob/master/CONTRIBUTING.md) ([_docs_](https://clomonitor.io/docs/topics/checks/#contributing))
          - [ ] Governance ([_docs_](https://clomonitor.io/docs/topics/checks/#governance))
          - [x] [Maintainers](https://github.com/artifacthub/hub/blob/master/OWNERS) ([_docs_](https://clomonitor.io/docs/topics/checks/#maintainers))
          - [x] [Readme](https://github.com/artifacthub/hub/blob/master/README.md) ([_docs_](https://clomonitor.io/docs/topics/checks/#readme))
          - [ ] Roadmap ([_docs_](https://clomonitor.io/docs/topics/checks/#roadmap))
          - [x] [Website](https://artifacthub.io) ([_docs_](https://clomonitor.io/docs/topics/checks/#website))

        ### License [100%]

          - [x] Apache-2.0 ([_docs_](https://clomonitor.io/docs/topics/checks/#spdx-id))
          - [x] Approved license ([_docs_](https://clomonitor.io/docs/topics/checks/#approved-license))
          - [x] [License scanning](https://app.fossa.io/projects/git%2Bhttps%3A%2F%2Fgithub.com%2Fartifacthub%2Fhub?ref=badge_shield) ([_docs_](https://clomonitor.io/docs/topics/checks/#license-scanning))

        ### Best Practices [95%]

          - [ ] Analytics ([_docs_](https://clomonitor.io/docs/topics/checks/#analytics))
          - [x] [Artifact Hub badge](https://artifacthub.io/packages/helm/artifact-hub/artifact-hub) ([_docs_](https://clomonitor.io/docs/topics/checks/#artifact-hub-badge))
          - [x] Contributor License Agreement ([_docs_](https://clomonitor.io/docs/topics/checks/#contributor-license-agreement)) \`EXEMPT\`
          - [x] Community meeting ([_docs_](https://clomonitor.io/docs/topics/checks/#community-meeting))
          - [x] Developer Certificate of Origin ([_docs_](https://clomonitor.io/docs/topics/checks/#developer-certificate-of-origin))
          - [x] [Github discussions](https://github.com/artifacthub/hub/discussions/2621) ([_docs_](https://clomonitor.io/docs/topics/checks/#github-discussions))
          - [x] [OpenSSF badge](https://bestpractices.coreinfrastructure.org/projects/4106) ([_docs_](https://clomonitor.io/docs/topics/checks/#openssf-badge))
          - [x] [Recent release](https://github.com/artifacthub/hub/releases/tag/v1.11.0) ([_docs_](https://clomonitor.io/docs/topics/checks/#recent-release))
          - [x] Slack precense ([_docs_](https://clomonitor.io/docs/topics/checks/#slack-presence))

        ### Security [100%]

          - [x] Binary artifacts ([_docs_](https://clomonitor.io/docs/topics/checks/#binary-artifacts-from-openssf-scorecard))
          - [x] Code review ([_docs_](https://clomonitor.io/docs/topics/checks/#code-review-from-openssf-scorecard))
          - [x] Dangerous workflow ([_docs_](https://clomonitor.io/docs/topics/checks/#dangerous-workflow-from-openssf-scorecard))
          - [x] Dependency update tool ([_docs_](https://clomonitor.io/docs/topics/checks/#dependency-update-tool-from-openssf-scorecard))
          - [x] Maintained ([_docs_](https://clomonitor.io/docs/topics/checks/#maintained-from-openssf-scorecard))
          - [x] Software bill of materials (SBOM) ([_docs_](https://clomonitor.io/docs/topics/checks/#software-bill-of-materials-sbom))
          - [x] [Security policy](https://github.com/artifacthub/hub/blob/master/SECURITY.md) ([_docs_](https://clomonitor.io/docs/topics/checks/#security-policy))
          - [x] Signed releases ([_docs_](https://clomonitor.io/docs/topics/checks/#signed-releases-from-openssf-scorecard))
          - [x] Token permissions ([_docs_](https://clomonitor.io/docs/topics/checks/#token-permissions-from-openssf-scorecard))

        ### Legal [100%]

          - [x] Trademark disclaimer ([_docs_](https://clomonitor.io/docs/topics/checks/#trademark-disclaimer)) \`EXEMPT\`

        For more information about the checks sets available and how each of the checks work, please see the [CLOMonitor's documentation](https://clomonitor.io/docs/topics/checks/).
        `;
        fetchMock.mockResponse(reportSample, {
          headers: {
            'content-type': 'text/plain; charset=utf-8',
          },
          status: 200,
        });

        const response = await API.getRepositoryReportMD('foundation', 'proj', 'repo');

        expect(fetchMock).toHaveBeenCalledTimes(1);
        expect(fetchMock.mock.calls[0][0]).toEqual('/api/projects/foundation/proj/repo/report.md');
        expect(response).toEqual(reportSample);
      });
    });

    describe('trackView', () => {
      it('success', async () => {
        fetchMock.mockResponse('', {
          headers: {
            'content-type': 'text/plain; charset=utf-8',
          },
          status: 204,
        });

        const response = await API.trackView('projectID');

        expect(fetchMock).toHaveBeenCalledTimes(1);
        expect(fetchMock.mock.calls[0][0]).toEqual('/api/projects/views/projectID');
        expect(fetchMock.mock.calls[0][1]!.method).toBe('POST');
        expect(response).toBe('');
      });
    });
  });
});
