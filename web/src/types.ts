export interface Organization {
  name: string;
  displayName?: string;
  description?: string;
  homeUrl: string;
  logoUrl?: string;
}

export interface BaseProject {
  id: string;
  name: string;
  displayName?: string;
  description?: string;
  homeUrl: string;
  logoUrl?: string;
  devstatsUrl?: string;
  maturityId: Maturity;
  categoryId: Category;
  score: Score;
  updatedAt: number;
}

export interface Project extends BaseProject {
  repositories: BaseRepository[];
  organization: {
    name: string;
  };
}

export interface ProjectDetail extends BaseProject {
  repositories: Repository[];
}

export interface BaseRepository {
  name: string;
  url: string;
  kind: RepositoryKind;
}

export interface Repository extends BaseRepository {
  digest: string;
  repositoryId: string;
  score: Score;
  reports: Report[];
}

export interface Report {
  data: CoreReport | any;
  linterId: LinterId;
  reportId: string;
  updatedAt: number;
}

export interface CoreReport {
  // ScoreType
  [key: string]: {
    [key: string]: number | boolean;
  };
}

export type Score = {
  [key in ScoreType]: number;
} & { scoreKind: ScoreKind };

export interface FiltersSection {
  name: string;
  title: string;
  filters: Filter[];
}

export interface Filter {
  name: string | number;
  label: string;
  legend?: string;
  decorator?: JSX.Element;
}

export interface Issue {
  level: number;
  description: string;
}

export interface Prefs {
  search: { limit: number; sort: { by: SortBy; direction: SortDirection } };
  theme: {
    effective: string;
  };
}

export interface ReportOptionData {
  icon: JSX.Element;
  name: string;
  legend: JSX.Element;
  description: JSX.Element;
}

export enum Maturity {
  Graduated = 0,
  Incubating,
  Sandbox,
}

export enum Category {
  'App definition' = 0,
  Observability,
  Orchestration,
  Platform,
  Provisioning,
  Runtime,
  Serverless,
}

export enum Rating {
  A = 'a',
  B = 'b',
  C = 'c',
  D = 'd',
}

export enum FilterKind {
  Maturity = 'maturity',
  Category = 'category',
  Rating = 'rating',
}

export enum LinterId {
  core = 0,
}

export enum ScoreType {
  Documentation = 'documentation',
  License = 'license',
  BestPractices = 'bestPractices',
  Security = 'security',
  Global = 'global',
}

export enum SortDirection {
  ASC = 'asc',
  DESC = 'desc',
}

export enum SortBy {
  Name = 'name',
  Score = 'score',
}

export enum RepositoryKind {
  Primary = 'primary',
  Secondary = 'secondary',
}

export enum ScoreKind {
  Primary = 'primary',
  Secondary = 'secondary',
}

export enum ReportOption {
  Adopters = 'adopters',
  Changelog = 'changelog',
  CodeOfConduct = 'codeOfConduct',
  Contributing = 'contributing',
  Governance = 'governance',
  Maintainers = 'maintainers',
  Readme = 'readme',
  Roadmap = 'roadmap',
  ApprovedLicense = 'approved',
  SPDX = 'spdxId',
  FossaBadge = 'fossaBadge',
  OpenSSFBadge = 'openssfBadge',
  SecurityPolicy = 'securityPolicy',
  CommunityMeeting = 'communityMeeting',
  ArtifactHubBadge = 'artifacthubBadge',
  Website = 'website',
}

export interface SearchFiltersURL extends BasicQuery {
  pageNumber: number;
}

export interface BasicQuery {
  text?: string;
  filters?: {
    [key: string]: (string | number)[];
  };
}

export interface SearchQuery extends BasicQuery {
  limit: number;
  offset: number;
  sortBy: SortBy;
  sortDirection: SortDirection;
}

export interface SearchData {
  limit: number;
  offset: number;
  sort_by: string;
  sort_direction: string;
  text?: string;
  category?: number[];
  maturity?: number[];
  rating?: number[];
}

export interface Error {
  kind: ErrorKind;
  message?: string;
}

export enum ErrorKind {
  Other,
  NotFound,
}

export type ReportOptionInfo = {
  [key in ReportOption]: ReportOptionData;
};
