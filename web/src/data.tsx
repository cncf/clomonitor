import { BiLock, BiMedal, BiShieldQuarter, BiTrophy, BiWorld } from 'react-icons/bi';
import { CgFileDocument, CgReadme } from 'react-icons/cg';
import { FaBalanceScale, FaCheckDouble, FaTools } from 'react-icons/fa';
import { FiHexagon } from 'react-icons/fi';
import { GiFountainPen, GiStamper, GiTiedScroll } from 'react-icons/gi';
import { GoLaw } from 'react-icons/go';
import { HiOutlinePencilAlt, HiTerminal } from 'react-icons/hi';
import { ImOffice } from 'react-icons/im';
import { IoIosPeople, IoMdRibbon } from 'react-icons/io';
import { RiRoadMapLine } from 'react-icons/ri';

import ExternalLink from './layout/common/ExternalLink';
import RoundedBadge from './layout/common/RoundedBadge';
import {
  Category,
  FilterKind,
  FiltersSection,
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
    name: FilterKind.Maturity,
    title: 'Maturity level',
    filters: [
      { name: Maturity.Graduated, label: 'Graduated' },
      { name: Maturity.Incubating, label: 'Incubating' },
      { name: Maturity.Sandbox, label: 'Sandbox' },
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
        decorator: <RoundedBadge level={1} />,
      },
      {
        name: Rating.B,
        label: 'B',
        legend: '[50-74]',
        decorator: <RoundedBadge level={2} />,
      },
      {
        name: Rating.C,
        label: 'C',
        legend: '[25-49]',
        decorator: <RoundedBadge level={3} />,
      },
      {
        name: Rating.D,
        label: 'D',
        legend: '[0-24]',
        decorator: <RoundedBadge level={4} />,
      },
    ],
  },
  {
    name: FilterKind.Category,
    title: 'Category',
    filters: [
      { name: Category['App definition'], label: 'App definition' },
      { name: Category.Observability, label: 'Observability' },
      { name: Category.Orchestration, label: 'Orchestration' },
      { name: Category.Platform, label: 'Platform' },
      { name: Category.Provisioning, label: 'Provisioning' },
      { name: Category.Runtime, label: 'Runtime' },
      { name: Category.Serverless, label: 'Serverless' },
    ],
  },
];

export const CATEGORY_ICONS = {
  [ScoreType.Global]: <BiTrophy />,
  [ScoreType.Documentation]: <HiOutlinePencilAlt />,
  [ScoreType.License]: <GoLaw />,
  [ScoreType.BestPractices]: <IoMdRibbon />,
  [ScoreType.Security]: <BiLock />,
};

export const REPORT_OPTIONS: ReportOptionInfo = {
  [ReportOption.Adopters]: {
    icon: <ImOffice />,
    name: 'Adopters',
    legend: <span>List of organizations using this project in production or at stages of testing</span>,
    description: (
      <span>
        We check that an <code>adopters*</code> <em>(no case sensitive)</em> file exists at the <code>root</code> of the
        repository or that the <code>README</code> file contains an <strong>adopters</strong> header
      </span>
    ),
  },
  [ReportOption.Changelog]: {
    icon: <CgFileDocument />,
    name: 'Changelog',
    legend: <span>A curated, chronologically ordered list of notable changes for each version</span>,
    description: (
      <span>
        We check that an <code>changelog*</code> <em>(no case sensitive)</em> file exists at the <code>root</code> of
        the repository
      </span>
    ),
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
    description: (
      <span>
        We check that a <code>code*of*conduct.md*</code> <em>(no case sensitive)</em> file exists at the{' '}
        <code>root</code> of the repository or in the <code>docs</code> directory or that the <code>README</code> file
        contains a <strong>code of conduct</strong> header
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
    description: (
      <span>
        We check that a <code>contributing*</code> <em>(no case sensitive)</em> file exists at the <code>root</code> of
        the repository or in the <code>docs</code> directory
      </span>
    ),
  },
  [ReportOption.Governance]: {
    icon: <GiTiedScroll />,
    name: 'Governance',
    legend: <span>Document that explains how the governance and committer process works in the repository</span>,
    description: (
      <span>
        We check that a <code>governance*</code> <em>(no case sensitive)</em> file exists at the <code>root</code> of
        the repository or in the <code>docs</code> directory or that the <code>README</code> file contains a
        <strong>governance</strong> header
      </span>
    ),
  },
  [ReportOption.Maintainers]: {
    icon: <FaTools />,
    name: 'Maintainers',
    legend: (
      <span>
        The <em>maintainers</em> file contains a list of the current maintainers of the repository
      </span>
    ),
    description: (
      <span>
        We check that a <code>maintainers*</code>, <code>owners*</code> or <code>codeowners*</code>{' '}
        <em>(no case sensitive)</em> file exists at the <code>root</code> of the repository or in the <code>docs</code>{' '}
        directory
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
    description: (
      <span>
        We check that a <code>README*</code> <em>(case sensitive)</em> file exists at the <code>root</code> of the
        repository
      </span>
    ),
  },
  [ReportOption.Roadmap]: {
    icon: <RiRoadMapLine />,
    name: 'Roadmap',
    legend: (
      <span>Defines a high-level overview of the project's goals and deliverables ideally presented on a timeline</span>
    ),
    description: (
      <span>
        We check that a <code>roadmap*</code> <em>(no case sensitive)</em> file exists at the <code>root</code> of the
        repository or that the <code>README</code> file contains a <strong>roadmap</strong> header
      </span>
    ),
  },
  [ReportOption.ApprovedLicense]: {
    icon: <FaCheckDouble />,
    name: 'Approved license',
    legend: <span>Whether the repository uses an approved license or not.</span>,
    description: (
      <span>
        If the repository&#39;s license has been identified, this check verifies that it is one of the approved licenses{' '}
        <em>
          (Apache-2.0, BSD-2-Clause, BSD-2-Clause-FreeBSD, BSD-3-Clause, ISC, MIT, PostgreSQL, Python-2.0, X11, Zlib)
        </em>
      </span>
    ),
  },
  [ReportOption.SPDX]: {
    icon: <FaBalanceScale />,
    name: 'License',
    legend: (
      <span>
        The <em>LICENSE</em> file contains the repository's license
      </span>
    ),
    description: (
      <span>
        We process the <code>LICENSE*</code> or <code>COPYING*</code> <em>(case sensitive)</em> file at the{' '}
        <code>root</code> of the repository and try to detect the license <code>SPDX</code> identifier from the file
        content
      </span>
    ),
  },
  [ReportOption.FossaBadge]: {
    icon: <GiStamper />,
    name: 'FOSSA badge',
    legend: (
      <span>
        <em>FOSSA</em> scans and automatically identifies, manages and addresses open source licensing issues and
        security vulnerabilities
      </span>
    ),
    description: (
      <span>
        We check that the <code>README</code> file contains a <em>FOSSA</em> badge
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
    description: (
      <span>
        We check that the <code>README</code> file contains a <em>OpenSSF (CII)</em> badge
      </span>
    ),
  },
  [ReportOption.SecurityPolicy]: {
    icon: <BiShieldQuarter />,
    name: 'Security policy',
    legend: <span>Clearly documented security processes explaining how to report security issues to the project</span>,
    description: (
      <span>
        We check that a <code>security*</code> <em>(no case sensitive)</em> file exists at the <code>root</code> of the
        repository, in the <code>docs</code> or <code>.github</code> directories or that the <code>README</code> file
        contains a <strong>security</strong> header
      </span>
    ),
  },
  [ReportOption.CommunityMeeting]: {
    icon: <IoIosPeople />,
    name: 'Community meeting',
    legend: (
      <span>
        Community meetings are often held to engage community members, hear more voices and get more viewpoints
      </span>
    ),
    description: (
      <span>
        We check that the <code>README</code> file contains patterns like <em>community meeting</em>,{' '}
        <em>meeting minutes</em>, etc.
      </span>
    ),
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
    description: (
      <span>
        We check that the <code>README</code> file contains an Artifact Hub badge
      </span>
    ),
  },
  [ReportOption.Website]: {
    icon: <BiWorld />,
    name: 'Website',
    legend: <span>A url that users can visit to learn more about your project</span>,
    description: <span>We check that the repository has a website set in Github</span>,
  },
};
