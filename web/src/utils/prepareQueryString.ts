import { isEmpty, isUndefined } from 'lodash';

import { BasicQuery, SearchFiltersURL, SearchQuery } from '../types';

export const getURLSearchParams = (query: BasicQuery): URLSearchParams => {
  const q = new URLSearchParams();
  if (!isUndefined(query.filters) && !isEmpty(query.filters)) {
    Object.keys(query.filters).forEach((filterId: string) => {
      return query.filters![filterId].forEach((id: string | number) => {
        q.append(filterId, id.toString());
      });
    });
  }
  if (!isUndefined(query.text)) {
    q.set('text', query.text);
  }
  return q;
};

export const prepareAPIQueryString = (query: SearchQuery): string => {
  const q = getURLSearchParams(query);
  q.set('limit', query.limit.toString());
  q.set('offset', query.offset.toString());
  return `?${q.toString()}`;
};

export const prepareQueryString = (query: SearchFiltersURL): string => {
  const q = getURLSearchParams(query);
  q.set('page', query.pageNumber.toString());
  return `?${q.toString()}`;
};
