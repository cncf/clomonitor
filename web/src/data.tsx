import RoundedBadge from './layout/common/RoundedBadge';
import { Category, FilterKind, FiltersSection, Maturity, Rating } from './types';

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
