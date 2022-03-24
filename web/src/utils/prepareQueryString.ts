import { isEmpty, isUndefined } from 'lodash';

import { BasicQuery, SearchFiltersURL } from '../types';

const getURLSearchParams = (query: BasicQuery): URLSearchParams => {
  const q = new URLSearchParams();
  if (!isUndefined(query.filters) && !isEmpty(query.filters)) {
    Object.keys(query.filters).forEach((filterId: string) => {
      return query.filters![filterId].forEach((id: string | number) => {
        q.append(filterId, id.toString());
      });
    });
  }
  if (!isUndefined(query.text) && query.text !== '') {
    q.set('text', query.text);
  }
  if (!isUndefined(query.accepted_from)) {
    q.set('accepted_from', query.accepted_from);
  }
  if (!isUndefined(query.accepted_to)) {
    q.set('accepted_to', query.accepted_to);
  }
  return q;
};

const prepareQueryString = (query: SearchFiltersURL): string => {
  const q = getURLSearchParams(query);
  q.set('page', query.pageNumber.toString());
  return `?${q.toString()}`;
};

export default prepareQueryString;
