import { render, screen, waitFor } from '@testing-library/react';
import { mocked } from 'jest-mock';
import ReactRouter, { BrowserRouter as Router } from 'react-router-dom';

import API from '../../../api';
import { AppContext } from '../../../context/AppContextProvider';
import { SortBy, SortDirection } from '../../../types';
import RepositoryReportModal from './RepositoryReportModal';

jest.mock('../../../api');
jest.mock('react-router-dom', () => ({
  ...(jest.requireActual('react-router-dom') as any),
  useParams: jest.fn(),
  useLocation: jest.fn(),
}));

const mockCtx = {
  prefs: {
    search: { limit: 20, sort: { by: SortBy.Name, direction: SortDirection.ASC } },
    theme: { effective: 'light', configured: 'light' },
  },
};

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

const mockOnCloseModal = jest.fn();

const defaultProps = {
  repoName: 'repo',
  openStatus: true,
  onCloseModal: mockOnCloseModal,
};

describe('RepositoryReportModal', () => {
  beforeEach(() => {
    jest.spyOn(ReactRouter, 'useParams').mockReturnValue({ project: 'proj', foundation: 'cncf' });
    jest.spyOn(ReactRouter, 'useLocation').mockReturnValue({
      pathname: '/projects/cncf/artifact-hub/artifact-hub',
      search: '',
      hash: '',
      state: { currentSearch: '?maturity=sandbox&rating=a&page=1' },
      key: 'key',
    });
  });

  afterEach(() => {
    jest.resetAllMocks();
  });

  it('creates snapshot', async () => {
    mocked(API).getRepositoryReportMD.mockResolvedValue(reportSample);
    const { asFragment } = render(
      <AppContext.Provider value={{ ctx: mockCtx, dispatch: jest.fn() }}>
        <Router>
          <RepositoryReportModal {...defaultProps} />
        </Router>
      </AppContext.Provider>
    );

    await waitFor(() => {
      expect(API.getRepositoryReportMD).toHaveBeenCalledTimes(1);
    });

    expect(await screen.findByText('Repository report')).toBeInTheDocument();

    expect(asFragment()).toMatchSnapshot();
  });

  describe('Render', () => {
    it('renders proper content', async () => {
      mocked(API).getRepositoryReportMD.mockResolvedValue(reportSample);
      render(
        <AppContext.Provider value={{ ctx: mockCtx, dispatch: jest.fn() }}>
          <Router>
            <RepositoryReportModal {...defaultProps} />
          </Router>
        </AppContext.Provider>
      );

      await waitFor(() => {
        expect(API.getRepositoryReportMD).toHaveBeenCalledTimes(1);
        expect(API.getRepositoryReportMD).toHaveBeenCalledWith('cncf', 'proj', 'repo');
      });

      expect(await screen.findByText('Repository report')).toBeInTheDocument();
      expect(screen.getByRole('button', { name: 'Copy to clipboard' })).toBeInTheDocument();
    });

    it('displays loading while getting markdown', async () => {
      mocked(API).getRepositoryReportMD.mockResolvedValue(reportSample);

      render(
        <AppContext.Provider value={{ ctx: mockCtx, dispatch: jest.fn() }}>
          <Router>
            <RepositoryReportModal {...defaultProps} />
          </Router>
        </AppContext.Provider>
      );

      expect(screen.getByRole('status')).toBeInTheDocument();

      await waitFor(() => {
        expect(API.getRepositoryReportMD).toHaveBeenCalledTimes(1);
      });

      expect(await screen.findByText('Repository report')).toBeInTheDocument();
    });
  });
});
