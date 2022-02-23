import { isUndefined, sampleSize } from 'lodash';
import { Fragment, memo } from 'react';
import { Link } from 'react-router-dom';

import prepareQueryString from '../../utils/prepareQueryString';

interface Props {
  className?: string;
  lineBreakIn?: number;
}

interface SampleQuery {
  name: string;
  filters: any;
}

const QUERIES: SampleQuery[] = [
  {
    name: 'Only graduated projects',
    filters: {
      pageNumber: 1,
      filters: { maturity: [0] },
    },
  },
  {
    name: 'Only incubating projects',
    filters: {
      pageNumber: 1,
      filters: { maturity: [1] },
    },
  },
  {
    name: 'Only sandbox projects',
    filters: {
      pageNumber: 1,
      filters: { maturity: [2] },
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
    name: 'Projects in app definition category',
    filters: {
      pageNumber: 1,
      filters: { category: [0] },
    },
  },

  {
    name: 'Projects in app definition category',
    filters: {
      pageNumber: 1,
      filters: { category: [0] },
    },
  },
  {
    name: 'Projects in observability category',
    filters: {
      pageNumber: 1,
      filters: { category: [1] },
    },
  },
  {
    name: 'Projects in orchestration category',
    filters: {
      pageNumber: 1,
      filters: { category: [2] },
    },
  },
  {
    name: 'Projects in platform category',
    filters: {
      pageNumber: 1,
      filters: { category: [3] },
    },
  },
  {
    name: 'Projects in provisioning category',
    filters: {
      pageNumber: 1,
      filters: { category: [4] },
    },
  },
  {
    name: 'Projects in runtime category',
    filters: {
      pageNumber: 1,
      filters: { category: [5] },
    },
  },
  {
    name: 'Projects in serverless category',
    filters: {
      pageNumber: 1,
      filters: { category: [6] },
    },
  },
];

const QUERIES_NUMBER = 5;

const SampleQueries = (props: Props) => {
  const queries = QUERIES.length > QUERIES_NUMBER ? sampleSize(QUERIES, QUERIES_NUMBER) : QUERIES;

  return (
    <>
      {queries.map((query: SampleQuery, index: number) => (
        <Fragment key={`sampleQuery_${index}`}>
          <Link
            className={`badge rounded-0 border fw-normal mx-2 mt-3 text-decoration-none ${props.className}`}
            to={{
              pathname: '/search',
              search: prepareQueryString(query.filters),
            }}
            aria-label={`Filter by ${query.name}`}
          >
            {query.name}
          </Link>
          {!isUndefined(props.lineBreakIn) && index === props.lineBreakIn - 1 && (
            <div className="d-block w-100" data-testid="sampleQueryBreakLine" />
          )}
        </Fragment>
      ))}
    </>
  );
};

export default memo(SampleQueries);
