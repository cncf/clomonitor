import { ExternalLink, Foundation, Maturity, SampleQuery } from 'clo-ui';
import { BiLock, BiMedal, BiShieldQuarter, BiTable, BiTrophy, BiWorld } from 'react-icons/bi';
import { BsCalendar3, BsUiChecks } from 'react-icons/bs';
import { CgFileDocument, CgReadme } from 'react-icons/cg';
import {
  FaBalanceScale,
  FaCheckDouble,
  FaExclamationTriangle,
  FaFileContract,
  FaFileSignature,
  FaRobot,
  FaSignature,
  FaSlack,
  FaTools,
  FaTrademark,
  FaUserCog,
  FaUserSecret,
} from 'react-icons/fa';
import { FiHexagon } from 'react-icons/fi';
import { GiFountainPen, GiStamper, GiTiedScroll } from 'react-icons/gi';
import { GoCommentDiscussion, GoFileBinary, GoLaw } from 'react-icons/go';
import { GrDocumentLocked, GrDocumentText } from 'react-icons/gr';
import { HiOutlinePencilAlt, HiTerminal } from 'react-icons/hi';
import { ImOffice } from 'react-icons/im';
import { IoIosPeople, IoMdRibbon } from 'react-icons/io';
import { MdOutlineInventory, MdPreview } from 'react-icons/md';
import { RiRoadMapLine, RiShieldStarLine } from 'react-icons/ri';

import QualityDot from './layout/common/QualityDot';
import {
  ChecksPerCategory,
  FilterKind,
  FiltersSection,
  MaturityFilters,
  Rating,
  ReportOption,
  ReportOptionInfo,
  ScoreType,
  SortBy,
  SortDirection,
  SortOption,
} from './types';

export const FOUNDATIONS: FoundationInfo = {
  [Foundation.cdf]: {
    name: 'CDF',
  },
  [Foundation.cncf]: {
    name: 'CNCF',
  },
  [Foundation.lfaidata]: {
    name: 'LF AI & Data',
  },
  [Foundation.lfnetworking]: {
    name: 'LF Networking',
  },
  [Foundation.hyperledger]: {
    name: 'Hyperledger',
  },
};

export const DEFAULT_FOUNDATION = Foundation.cncf;

export const DEFAULT_SORT_BY = SortBy.Name;
export const DEFAULT_SORT_DIRECTION = SortDirection.ASC;

export const FILTERS: FiltersSection[] = [
  {
    name: FilterKind.Foundation,
    title: 'Foundation',
    filters: Object.keys(FOUNDATIONS).map((f: string) => {
      return { name: f, label: FOUNDATIONS[f as Foundation]!.name };
    }),
  },
  {
    name: FilterKind.Rating,
    title: 'Rating',
    filters: [
      {
        name: Rating.A,
        label: 'A',
        legend: '[75-100]',
        decorator: <QualityDot level={1} />,
      },
      {
        name: Rating.B,
        label: 'B',
        legend: '[50-74]',
        decorator: <QualityDot level={2} />,
      },
      {
        name: Rating.C,
        label: 'C',
        legend: '[25-49]',
        decorator: <QualityDot level={3} />,
      },
      {
        name: Rating.D,
        label: 'D',
        legend: '[0-24]',
        decorator: <QualityDot level={4} />,
      },
    ],
  },
];

export const MATURITY_FILTERS: MaturityFilters = {
  [Foundation.cdf]: {
    name: FilterKind.Maturity,
    title: 'Maturity level',
    filters: [
      { name: Maturity.graduated, label: 'Graduated' },
      { name: Maturity.incubating, label: 'Incubating' },
    ],
  },
  [Foundation.cncf]: {
    name: FilterKind.Maturity,
    title: 'Maturity level',
    filters: [
      { name: Maturity.graduated, label: 'Graduated' },
      { name: Maturity.incubating, label: 'Incubating' },
      { name: Maturity.sandbox, label: 'Sandbox' },
    ],
  },
  [Foundation.lfaidata]: {
    name: FilterKind.Maturity,
    title: 'Maturity level',
    filters: [
      { name: Maturity.graduated, label: 'Graduated' },
      { name: Maturity.incubating, label: 'Incubating' },
      { name: Maturity.sandbox, label: 'Sandbox' },
    ],
  },
  [Foundation.lfnetworking]: {
    name: FilterKind.Maturity,
    title: 'Maturity level',
    filters: [{ name: Maturity.lfn, label: 'LFN' }],
  },
  [Foundation.hyperledger]: {
    name: FilterKind.Maturity,
    title: 'Maturity level',
    filters: [
      { name: Maturity.graduated, label: 'Graduated' },
      { name: Maturity.incubating, label: 'Incubating' },
    ],
  },
};

export const SORT_OPTIONS: SortOption[] = [
  {
    label: 'Alphabetically (A-Z)',
    by: SortBy.Name,
    direction: SortDirection.ASC,
  },
  {
    label: 'Alphabetically (Z-A)',
    by: SortBy.Name,
    direction: SortDirection.DESC,
  },
  {
    label: 'Score (highest first)',
    by: SortBy.Score,
    direction: SortDirection.DESC,
  },
  {
    label: 'Score (lowest first)',
    by: SortBy.Score,
    direction: SortDirection.ASC,
  },
];

export const QUERIES: SampleQuery[] = [
  {
    name: 'Only graduated projects',
    filters: {
      pageNumber: 1,
      filters: { maturity: ['graduated'] },
    },
  },
  {
    name: 'Only incubating projects',
    filters: {
      pageNumber: 1,
      filters: { maturity: ['incubating'] },
    },
  },
  {
    name: 'Only sandbox projects',
    filters: {
      pageNumber: 1,
      filters: { maturity: ['sandbox'] },
    },
  },
  {
    name: 'Projects with A rating',
    filters: {
      pageNumber: 1,
      filters: { rating: ['a'] },
    },
  },
  {
    name: 'Projects with B rating',
    filters: {
      pageNumber: 1,
      filters: { rating: ['b'] },
    },
  },
  {
    name: 'Projects with C rating',
    filters: {
      pageNumber: 1,
      filters: { rating: ['c'] },
    },
  },
  {
    name: 'Projects with D rating',
    filters: {
      pageNumber: 1,
      filters: { rating: ['d'] },
    },
  },
  {
    name: 'Projects accepted by CNCF',
    filters: {
      pageNumber: 1,
      filters: { foundation: ['cncf'] },
    },
  },
  {
    name: 'Projects accepted by LF AI & Data',
    filters: {
      pageNumber: 1,
      filters: { category: ['lfaidata'] },
    },
  },
  {
    name: 'Projects accepted by CDF',
    filters: {
      pageNumber: 1,
      filters: { category: ['cdf'] },
    },
  },
];

export const CATEGORY_ICONS = {
  [ScoreType.BestPractices]: <RiShieldStarLine />,
  [ScoreType.Documentation]: <HiOutlinePencilAlt />,
  [ScoreType.Global]: <BiTrophy />,
  [ScoreType.Legal]: <GoLaw />,
  [ScoreType.License]: <IoMdRibbon />,
  [ScoreType.Security]: <BiLock />,
};

export const CATEGORY_NAMES = {
  [ScoreType.BestPractices]: 'Best Practices',
  [ScoreType.Documentation]: 'Documentation',
  [ScoreType.Global]: 'Global',
  [ScoreType.Legal]: 'Legal',
  [ScoreType.License]: 'License',
  [ScoreType.Security]: 'Security',
};

export const FILTER_CATEGORY_NAMES = {
  [FilterKind.Foundation]: 'Foundation',
  [FilterKind.Maturity]: 'Maturity',
  [FilterKind.Rating]: 'Rating',
  [FilterKind.PassingCheck]: 'Passed',
  [FilterKind.NotPassingCheck]: 'Not passed',
};

export const REPORT_OPTIONS: ReportOptionInfo = {
  [ReportOption.Adopters]: {
    icon: <ImOffice />,
    name: 'Adopters',
    legend: <span>List of organizations using this project in production or at stages of testing</span>,
    reference: '/docs/topics/checks/#adopters',
  },
  [ReportOption.ApprovedLicense]: {
    icon: <FaCheckDouble />,
    name: 'Approved license',
    legend: <span>Whether the repository uses an approved license or not</span>,
    reference: '/docs/topics/checks/#approved-license',
  },
  [ReportOption.ArtifactHubBadge]: {
    icon: <FiHexagon />,
    name: 'Artifact Hub badge',
    legend: (
      <span>
        Projects can list their content on{' '}
        <ExternalLink className="d-inline-block text-decoration-underline" href="https://artifacthub.io">
          Artifact Hub
        </ExternalLink>{' '}
        to improve their discoverability
      </span>
    ),
    reference: '/docs/topics/checks/#artifact-hub-badge',
  },
  [ReportOption.BinaryArtifacts]: {
    icon: <GoFileBinary />,
    name: 'Binary artifacts',
    legend: <span>Whether the project has generated executable (binary) artifacts in the source repository</span>,
    reference: '/docs/topics/checks/#binary-artifacts-from-openssf-scorecard',
  },
  [ReportOption.Changelog]: {
    icon: <CgFileDocument />,
    name: 'Changelog',
    legend: <span>A curated, chronologically ordered list of notable changes for each version</span>,
    reference: '/docs/topics/checks/#changelog',
  },
  [ReportOption.CLA]: {
    icon: <FaFileContract />,
    name: 'Contributor License Agreement',
    shortName: 'CLA',
    legend: <span>Defines the terms under which intellectual property has been contributed to a company/project</span>,
    reference: '/docs/topics/checks/#contributor-license-agreement',
  },
  [ReportOption.CodeOfConduct]: {
    icon: <GiFountainPen />,
    name: 'Code of conduct',
    legend: (
      <span>
        Adopt a code of conduct to define community standards, signal a welcoming and inclusive project, and outline
        procedures for handling abuse
      </span>
    ),
    reference: '/docs/topics/checks/#code-of-conduct',
  },
  [ReportOption.CodeReview]: {
    icon: <MdPreview />,
    name: 'Code review',
    legend: <span>The project requires code review before pull requests (merge requests) are merged</span>,
    reference: '/docs/topics/checks/#code-review-from-openssf-scorecard',
  },
  [ReportOption.CommunityMeeting]: {
    icon: <IoIosPeople />,
    name: 'Community meeting',
    legend: (
      <span>
        Community meetings are often held to engage community members, hear more voices and get more viewpoints
      </span>
    ),
    reference: '/docs/topics/checks/#community-meeting',
  },
  [ReportOption.Contributing]: {
    icon: <HiTerminal />,
    name: 'Contributing',
    legend: (
      <span>
        A <em>contributing</em> file in your repository provides potential project contributors with a short guide to
        how they can help with your project
      </span>
    ),
    reference: '/docs/topics/checks/#contributing',
  },
  [ReportOption.DangerousWorkflow]: {
    icon: <FaExclamationTriangle />,
    name: 'Dangerous workflow',
    legend: <span>Whether the project's GitHub Action workflows has dangerous code patterns</span>,
    reference: '/docs/topics/checks/#dangerous-workflow-from-openssf-scorecard',
  },
  [ReportOption.DependenciesPolicy]: {
    icon: <GrDocumentText />,
    name: 'Dependencies policy',
    legend: <span>The project provides a policy that describes how dependencies are consumed and updated</span>,
    reference: '/docs/topics/checks/#dependencies-policy',
  },
  [ReportOption.DependencyUpdateTool]: {
    icon: <FaRobot />,
    name: 'Dependency update tool',
    legend: <span>The project uses a dependency update tool, specifically dependabot or renovatebot</span>,
    reference: '/docs/topics/checks/#dependency-update-tool-from-openssf-scorecard',
  },
  [ReportOption.DCO]: {
    icon: <FaFileSignature />,
    name: 'Developer Certificate of Origin',
    shortName: 'DCO',
    legend: (
      <span>
        Mechanism for contributors to certify that they wrote or have the right to submit the code they are contributing
      </span>
    ),
    reference: '/docs/topics/checks/#developer-certificate-of-origin',
  },
  [ReportOption.GithubDiscussions]: {
    icon: <GoCommentDiscussion />,
    name: 'GitHub discussions',
    legend: <span>Projects should enable discussions in their repositories</span>,
    reference: '/docs/topics/checks/#github-discussions',
  },
  [ReportOption.Governance]: {
    icon: <GiTiedScroll />,
    name: 'Governance',
    legend: <span>Document that explains how the governance and committer process works in the repository</span>,
    reference: '/docs/topics/checks/#governance',
  },
  [ReportOption.LicenseScanning]: {
    icon: <GiStamper />,
    name: 'License scanning',
    legend: (
      <span>
        License scanning software scans and automatically identifies, manages and addresses open source licensing issues
      </span>
    ),
    reference: '/docs/topics/checks/#license-scanning',
  },
  [ReportOption.Maintained]: {
    icon: <FaTools />,
    name: 'Maintained',
    legend: <span>Whether the project is actively maintained</span>,
    reference: '/docs/topics/checks/#maintained-from-openssf-scorecard',
  },
  [ReportOption.Maintainers]: {
    icon: <FaUserCog />,
    name: 'Maintainers',
    legend: (
      <span>
        The <em>maintainers</em> file contains a list of the current maintainers of the repository
      </span>
    ),
    reference: '/docs/topics/checks/#maintainers',
  },
  [ReportOption.OpenSSFBadge]: {
    icon: <BiMedal />,
    name: 'OpenSSF best practices badge',
    shortName: 'OpenSSF best practices',
    legend: (
      <span>
        The Open Source Security Foundation (OpenSSF) Best Practices badge is a way for Free/Libre and Open Source
        Software (FLOSS) projects to show that they follow best practices
      </span>
    ),
    reference: '/docs/topics/checks/#openssf-best-practices-badge',
  },
  [ReportOption.OpenSSFScorecardBadge]: {
    icon: <BsUiChecks />,
    name: 'OpenSSF Scorecard badge',
    shortName: 'OpenSSF Scorecard',
    legend: (
      <span>Scorecard assesses open source projects for security risks through a series of automated checks</span>
    ),
    reference: '/docs/topics/checks/#openssf-scorecard-badge',
  },
  [ReportOption.Readme]: {
    icon: <CgReadme />,
    name: 'Readme',
    legend: (
      <span>
        The <em>readme</em> file introduces and explains a project. It contains information that is commonly required to
        understand what the project is about
      </span>
    ),
    reference: '/docs/topics/checks/#readme',
  },
  [ReportOption.RecentRelease]: {
    icon: <BsCalendar3 />,
    name: 'Recent release',
    legend: <span>The project should have released at least one version in the last year</span>,
    reference: '/docs/topics/checks/#recent-release',
  },
  [ReportOption.Roadmap]: {
    icon: <RiRoadMapLine />,
    name: 'Roadmap',
    legend: (
      <span>Defines a high-level overview of the project's goals and deliverables ideally presented on a timeline</span>
    ),
    reference: '/docs/topics/checks/#roadmap',
  },
  [ReportOption.SBOM]: {
    icon: <MdOutlineInventory />,
    name: 'Software bill of materials (SBOM)',
    shortName: 'SBOM',
    legend: <span>List of components in a piece of software, including licenses, versions, etc</span>,
    reference: '/docs/topics/checks/#software-bill-of-materials-sbom',
  },
  [ReportOption.SecurityInsights]: {
    icon: <GrDocumentLocked />,
    name: 'Security insights',
    legend: (
      <span>
        The project provides an{' '}
        <ExternalLink
          className="d-inline-block text-decoration-underline"
          href="https://github.com/ossf/security-insights-spec/blob/v1.0.0/specification.md"
        >
          OpenSSF Security Insights
        </ExternalLink>{' '}
        manifest file
      </span>
    ),
    reference: '/docs/topics/checks/#security-insights',
  },
  [ReportOption.SecurityPolicy]: {
    icon: <BiShieldQuarter />,
    name: 'Security policy',
    legend: <span>Clearly documented security processes explaining how to report security issues to the project</span>,
    reference: '/docs/topics/checks/#security-policy',
  },
  [ReportOption.SignedReleases]: {
    icon: <FaSignature />,
    name: 'Signed releases',
    legend: <span>The project cryptographically signs release artifacts</span>,
    reference: '/docs/topics/checks/#signed-releases-from-openssf-scorecard',
  },
  [ReportOption.SlackPresence]: {
    icon: <FaSlack />,
    name: 'Slack presence',
    legend: <span>Projects should have presence in the CNCF Slack or Kubernetes Slack</span>,
    reference: '/docs/topics/checks/#slack-presence',
  },
  [ReportOption.SPDX]: {
    icon: <FaBalanceScale />,
    name: 'License found',
    legend: (
      <span>
        The <em>LICENSE</em> file contains the repository's license
      </span>
    ),
    reference: '/docs/topics/checks/#spdx-id',
  },
  [ReportOption.SummaryTable]: {
    icon: <BiTable />,
    name: 'Summary Table',
    legend: (
      <span>
        Projects should{' '}
        <ExternalLink
          href="https://github.com/cncf/landscape/blob/master/docs/item_summary.md"
          className="text-decoration-underline"
        >
          provide some information
        </ExternalLink>{' '}
        for the Landscape Summary Table
      </span>
    ),
    reference: '/docs/topics/checks/#summary-table',
  },
  [ReportOption.TokenPermissions]: {
    icon: <FaUserSecret />,
    name: 'Token permissions',
    legend: <span>Whether the project's automated workflows tokens are set to read-only by default</span>,
    reference: '/docs/topics/checks/#token-permissions-from-openssf-scorecard',
  },
  [ReportOption.TrademarkDisclaimer]: {
    icon: <FaTrademark />,
    name: 'Trademark disclaimer',
    legend: <span>Projects sites should have the Linux Foundation trademark disclaimer</span>,
    reference: '/docs/topics/checks/#trademark-disclaimer',
  },
  [ReportOption.Website]: {
    icon: <BiWorld />,
    name: 'Website',
    legend: <span>A url that users can visit to learn more about your project</span>,
    reference: '/docs/topics/checks/#website',
  },
};

export type FoundationInfo = {
  [key in Foundation]?: {
    name: string;
  };
};

export const CHECKS_PER_CATEGORY: ChecksPerCategory = {
  [ScoreType.Documentation]: [
    ReportOption.Adopters,
    ReportOption.Changelog,
    ReportOption.CodeOfConduct,
    ReportOption.Contributing,
    ReportOption.Governance,
    ReportOption.Maintainers,
    ReportOption.Readme,
    ReportOption.Roadmap,
    ReportOption.SummaryTable,
    ReportOption.Website,
  ],
  [ScoreType.License]: [ReportOption.SPDX, ReportOption.ApprovedLicense, ReportOption.LicenseScanning],
  [ScoreType.BestPractices]: [
    ReportOption.ArtifactHubBadge,
    ReportOption.CLA,
    ReportOption.CommunityMeeting,
    ReportOption.DCO,
    ReportOption.GithubDiscussions,
    ReportOption.OpenSSFBadge,
    ReportOption.OpenSSFScorecardBadge,
    ReportOption.RecentRelease,
    ReportOption.SlackPresence,
  ],
  [ScoreType.Security]: [
    ReportOption.BinaryArtifacts,
    ReportOption.CodeReview,
    ReportOption.DangerousWorkflow,
    ReportOption.DependencyUpdateTool,
    ReportOption.Maintained,
    ReportOption.SBOM,
    ReportOption.SecurityInsights,
    ReportOption.SecurityPolicy,
    ReportOption.SignedReleases,
    ReportOption.TokenPermissions,
  ],
  [ScoreType.Legal]: [ReportOption.TrademarkDisclaimer],
};
