import { BiLock, BiMedal, BiShieldQuarter, BiTrophy } from 'react-icons/bi';
import { CgFileDocument, CgReadme } from 'react-icons/cg';
import { FaBalanceScale, FaCheckDouble, FaTools } from 'react-icons/fa';
import { GiFountainPen, GiStamper, GiTiedScroll } from 'react-icons/gi';
import { GoLaw } from 'react-icons/go';
import { HiCode, HiOutlinePencilAlt, HiTerminal } from 'react-icons/hi';
import { ImOffice } from 'react-icons/im';
import { RiRoadMapLine } from 'react-icons/ri';

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
    title: 'Quality rating',
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
  [ScoreType.Quality]: <HiCode />,
  [ScoreType.Security]: <BiLock />,
};

export const REPORT_OPTIONS_BY_CATEGORY = {
  [ScoreType.Documentation]: [
    ReportOption.Readme,
    ReportOption.CodeOfConduct,
    ReportOption.Contributing,
    ReportOption.Governance,
    ReportOption.Adopters,
    ReportOption.Changelog,
    ReportOption.Maintainers,
    ReportOption.Roadmap,
  ],
  [ScoreType.License]: [ReportOption.SPDX, ReportOption.ApprovedLicense],
  [ScoreType.Quality]: [ReportOption.FossaBadge, ReportOption.OpenSSFBadge],
  [ScoreType.Security]: [ReportOption.SecurityPolicy],
};

export const REPORT_OPTIONS: ReportOptionInfo = {
  [ReportOption.Adopters]: {
    icon: <ImOffice />,
    name: 'Adopters',
    description: (
      <p className="mb-0">
        We check that an <code>adopters*</code> <em>(no case sensitive)</em> file exists at the <code>root</code> of the
        repository
      </p>
    ),
    weight: 5,
  },
  [ReportOption.Changelog]: {
    icon: <CgFileDocument />,
    name: 'Changelog',
    description: (
      <p className="mb-0">
        We check that an <code>changelog*</code> <em>(no case sensitive)</em> file exists at the <code>root</code> of
        the repository
      </p>
    ),
    weight: 5,
  },
  [ReportOption.CodeOfConduct]: {
    icon: <GiFountainPen />,
    name: 'Code of conduct',
    description: (
      <p className="mb-0">
        We check that a <code>code*of*conduct.md*</code> <em>(no case sensitive)</em> file exists at the{' '}
        <code>root</code> of the repository or in the <code>docs</code> directory
      </p>
    ),
    weight: 10,
  },
  [ReportOption.Contributing]: {
    icon: <HiTerminal />,
    name: 'Contributing',
    description: (
      <p className="mb-0">
        We check that a <code>contributing*</code> <em>(no case sensitive)</em> file exists at the <code>root</code> of
        the repository or in the <code>docs</code> directory
      </p>
    ),
    weight: 10,
  },
  [ReportOption.Governance]: {
    icon: <GiTiedScroll />,
    name: 'Governance',
    description: (
      <p className="mb-0">
        We check that a <code>governance*</code> <em>(no case sensitive)</em> file exists at the <code>root</code> of
        the repository or in the <code>docs</code> directory
      </p>
    ),
    weight: 10,
  },
  [ReportOption.Maintainers]: {
    icon: <FaTools />,
    name: 'Maintainers',
    description: (
      <p className="mb-0">
        We check that a <code>maintainers*</code>, <code>owners*</code> or <code>codeowners*</code>{' '}
        <em>(no case sensitive)</em> file exists at the <code>root</code> of the repository or in the <code>docs</code>{' '}
        directory
      </p>
    ),
    weight: 5,
  },
  [ReportOption.Readme]: {
    icon: <CgReadme />,
    name: 'Readme',
    description: (
      <p className="mb-0">
        We check that a <code>README*</code> <em>(case sensitive)</em> file exists at the <code>root</code> of the
        repository
      </p>
    ),
    weight: 50,
  },
  [ReportOption.Roadmap]: {
    icon: <RiRoadMapLine />,
    name: 'Roadmap',
    description: (
      <p className="mb-0">
        We check that a <code>roadmap*</code> <em>(no case sensitive)</em> file exists at the <code>root</code> of the
        repository
      </p>
    ),
    weight: 5,
  },
  [ReportOption.ApprovedLicense]: {
    icon: <FaCheckDouble />,
    name: 'Approved license',
    description: (
      <p className="mb-0">
        If the repository&#39;s license has been identified, this check verifies that it is one of the approved licenses{' '}
        <em>
          (Apache-2.0, BSD-2-Clause, BSD-2-Clause-FreeBSD, BSD-3-Clause, ISC, MIT, PostgreSQL, Python-2.0, X11, Zlib)
        </em>
      </p>
    ),
    weight: 75,
  },
  [ReportOption.SPDX]: {
    icon: <FaBalanceScale />,
    name: 'License',
    description: (
      <p className="mb-0">
        We process the <code>LICENSE*</code> or <code>COPYING*</code> <em>(case sensitive)</em> file at the{' '}
        <code>root</code> of the repository and try to detect the license <code>SPDX</code> identifier from the file
        content
      </p>
    ),
    weight: 25,
  },
  [ReportOption.FossaBadge]: {
    icon: <GiStamper />,
    name: 'FOSSA badge',
    description: (
      <p className="mb-0">
        We check that the <code>README</code> file contains a <em>FOSSA</em> badge
      </p>
    ),
    weight: 50,
  },
  [ReportOption.OpenSSFBadge]: {
    icon: <BiMedal />,
    name: 'OpenSSF badge',
    description: (
      <p className="mb-0">
        We check that the <code>README</code> file contains a <em>OpenSSF (CII)</em> badge
      </p>
    ),
    weight: 50,
  },
  [ReportOption.SecurityPolicy]: {
    icon: <BiShieldQuarter />,
    name: 'Security policy',
    description: (
      <p className="mb-0">
        We check that a <code>security*</code> <em>(no case sensitive)</em> file exists at the <code>root</code> of the
        repository or in the <code>docs</code> or <code>.github</code> directories
      </p>
    ),
    weight: 100,
  },
};
