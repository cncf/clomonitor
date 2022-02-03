export interface Organization {
  name: string;
  displayName?: string;
  description?: string;
  homeUrl: string;
  logoUrl?: string;
}

export interface Project {
  name: string;
  displayName?: string;
  description?: string;
  homeUrl: string;
  logoUrl?: string;
  devstatsUrl?: string;
  maturityId: Maturity;
  categoryId: Category;
  score: Score;
  repositories: Repository[];
  updatedAt: number;
}

export interface Repository {
  name: string;
  url: string;
}

export interface Report {
  data: any;
  errors?: string;
}

export type Score = {
  [key in ScoreType]: number;
};

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

export enum ScoreType {
  Documentation = 'documentation',
  License = 'license',
  Quality = 'quality',
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
