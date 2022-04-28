import { BiLock, BiMedal, BiShieldQuarter, BiTargetLock, BiTrophy, BiWorld } from 'react-icons/bi';
import { BsCalendar3 } from 'react-icons/bs';
import { CgFileDocument, CgReadme } from 'react-icons/cg';
import {
  FaBalanceScale,
  FaCheckDouble,
  FaCodeBranch,
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
import { GoFileBinary, GoLaw } from 'react-icons/go';
import { HiOutlinePencilAlt, HiTerminal } from 'react-icons/hi';
import { ImOffice } from 'react-icons/im';
import { IoIosPeople, IoMdRibbon } from 'react-icons/io';
import { MdOutlineInventory, MdPreview } from 'react-icons/md';
import { RiRoadMapLine, RiShieldStarLine } from 'react-icons/ri';

import ExternalLink from './layout/common/ExternalLink';
import QualityDot from './layout/common/QualityDot';
import {
  FilterKind,
  FiltersSection,
  Foundation,
  Maturity,
  Rating,
  ReportOption,
  ReportOptionInfo,
  ScoreType,
  SortBy,
  SortDirection,
} from './types';

export const DEFAULT_SORT_BY = SortBy.Name;
export const DEFAULT_SORT_DIRECTION = SortDirection.ASC;

export const FILTERS: FiltersSection[] = [
  {
    name: FilterKind.Foundation,
    title: 'Foundation',
    filters: [
      { name: Foundation.cncf, label: 'CNCF' },
      { name: Foundation.lfaidata, label: 'LF AI & Data' },
    ],
  },
  {
    name: FilterKind.Maturity,
    title: 'Maturity level',
    filters: [
      { name: Maturity.graduated, label: 'Graduated' },
      { name: Maturity.incubating, label: 'Incubating' },
      { name: Maturity.sandbox, label: 'Sandbox' },
    ],
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

export const CATEGORY_ICONS = {
  [ScoreType.BestPractices]: <RiShieldStarLine />,
  [ScoreType.Documentation]: <HiOutlinePencilAlt />,
  [ScoreType.Global]: <BiTrophy />,
  [ScoreType.Legal]: <GoLaw />,
  [ScoreType.License]: <IoMdRibbon />,
  [ScoreType.Security]: <BiLock />,
};

export const REPORT_OPTIONS: ReportOptionInfo = {
  [ReportOption.Adopters]: {
    icon: <ImOffice />,
    name: 'Adopters',
    legend: <span>List of organizations using this project in production or at stages of testing</span>,
  },
  [ReportOption.ApprovedLicense]: {
    icon: <FaCheckDouble />,
    name: 'Approved license',
    legend: <span>Whether the repository uses an approved license or not</span>,
  },
  [ReportOption.ArtifactHubBadge]: {
    icon: <FiHexagon />,
    name: 'Artifact Hub badge',
    legend: (
      <span>
        Projects can list their content on{' '}
        <ExternalLink className="d-inline-block" href="https://artifacthub.io">
          Artifact Hub
        </ExternalLink>{' '}
        to improve their discoverability
      </span>
    ),
  },
  [ReportOption.BinaryArtifacts]: {
    icon: <GoFileBinary />,
    name: 'Binary artifacts',
    legend: <span>Whether the project has generated executable (binary) artifacts in the source repository</span>,
  },
  [ReportOption.BranchProtection]: {
    icon: <FaCodeBranch />,
    name: 'Branch protection',
    legend: (
      <span>A project's default and release branches are protected with GitHub's branch protection settings</span>
    ),
  },
  [ReportOption.Changelog]: {
    icon: <CgFileDocument />,
    name: 'Changelog',
    legend: <span>A curated, chronologically ordered list of notable changes for each version</span>,
  },
  [ReportOption.CLA]: {
    icon: <FaFileContract />,
    name: 'Contributor License Agreement',
    shortName: 'CLA',
    legend: <span>Defines the terms under which intellectual property has been contributed to a company/project</span>,
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
  },
  [ReportOption.CodeReview]: {
    icon: <MdPreview />,
    name: 'Code review',
    legend: <span>The project requires code review before pull requests (merge requests) are merged</span>,
  },
  [ReportOption.CommunityMeeting]: {
    icon: <IoIosPeople />,
    name: 'Community meeting',
    legend: (
      <span>
        Community meetings are often held to engage community members, hear more voices and get more viewpoints
      </span>
    ),
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
  },
  [ReportOption.DangerousWorkflow]: {
    icon: <FaExclamationTriangle />,
    name: 'Dangerous workflow',
    legend: <span>Whether the project's GitHub Action workflows has dangerous code patterns</span>,
  },
  [ReportOption.DependencyUpdateTool]: {
    icon: <FaRobot />,
    name: 'Dependency update tool',
    legend: <span>The project uses a dependency update tool, specifically dependabot or renovatebot</span>,
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
  },
  [ReportOption.Governance]: {
    icon: <GiTiedScroll />,
    name: 'Governance',
    legend: <span>Document that explains how the governance and committer process works in the repository</span>,
  },
  [ReportOption.LicenseScanning]: {
    icon: <GiStamper />,
    name: 'License scanning',
    legend: (
      <span>
        License scanning software scans and automatically identifies, manages and addresses open source licensing issues
      </span>
    ),
  },
  [ReportOption.Maintained]: {
    icon: <FaTools />,
    name: 'Maintained',
    legend: <span>Whether the project is actively maintained</span>,
  },
  [ReportOption.Maintainers]: {
    icon: <FaUserCog />,
    name: 'Maintainers',
    legend: (
      <span>
        The <em>maintainers</em> file contains a list of the current maintainers of the repository
      </span>
    ),
  },
  [ReportOption.OpenSSFBadge]: {
    icon: <BiMedal />,
    name: 'OpenSSF badge',
    legend: (
      <span>
        The Open Source Security Foundation (OpenSSF) Best Practices badge is a way for Free/Libre and Open Source
        Software (FLOSS) projects to show that they follow best practices
      </span>
    ),
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
  },
  [ReportOption.RecentRelease]: {
    icon: <BsCalendar3 />,
    name: 'Recent release',
    legend: <span>The project should have released at least one version in the last year</span>,
  },
  [ReportOption.Roadmap]: {
    icon: <RiRoadMapLine />,
    name: 'Roadmap',
    legend: (
      <span>Defines a high-level overview of the project's goals and deliverables ideally presented on a timeline</span>
    ),
  },
  [ReportOption.SBOM]: {
    icon: <MdOutlineInventory />,
    name: 'Software bill of materials (SBOM)',
    shortName: 'SBOM',
    legend: <span>List of components in a piece of software, including licenses, versions, etc</span>,
  },
  [ReportOption.SecurityPolicy]: {
    icon: <BiShieldQuarter />,
    name: 'Security policy',
    legend: <span>Clearly documented security processes explaining how to report security issues to the project</span>,
  },
  [ReportOption.SignedReleases]: {
    icon: <FaSignature />,
    name: 'Signed releases',
    legend: <span>The project cryptographically signs release artifacts</span>,
  },
  [ReportOption.SlackPresence]: {
    icon: <FaSlack />,
    name: 'Slack presence',
    legend: <span>Projects should have presence in the CNCF Slack or Kubernetes Slack</span>,
  },
  [ReportOption.SPDX]: {
    icon: <FaBalanceScale />,
    name: 'License',
    legend: (
      <span>
        The <em>LICENSE</em> file contains the repository's license
      </span>
    ),
  },
  [ReportOption.TokenPermissions]: {
    icon: <FaUserSecret />,
    name: 'Token permissions',
    legend: <span>Whether the project's automated workflows tokens are set to read-only by default</span>,
  },
  [ReportOption.TrademarkDisclaimer]: {
    icon: <FaTrademark />,
    name: 'Trademark disclaimer',
    legend: <span>Projects sites should have the Linux Foundation trademark disclaimer</span>,
  },
  [ReportOption.Vulnerabilities]: {
    icon: <BiTargetLock />,
    name: 'Vulnerabilities',
    legend: (
      <span>
        Whether the project has open, unfixed vulnerabilities (uses the OSV -Open Source Vulnerabilities- service)
      </span>
    ),
  },
  [ReportOption.Website]: {
    icon: <BiWorld />,
    name: 'Website',
    legend: <span>A url that users can visit to learn more about your project</span>,
  },
};

export const FOUNDATIONS = {
  [Foundation.cncf]: {
    name: 'CNCF',
  },
  [Foundation.lfaidata]: {
    name: 'LF AI & Data',
  },
};
