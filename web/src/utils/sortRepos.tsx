import { orderBy } from 'lodash';

import { CheckSet, Repository } from '../types';
import getCheckSets from './getCheckSets';

// Sort by community repo kind, score global and alphabetically
const sortRepos = (repos: Repository[]): Repository[] => {
  return orderBy(
    repos,
    [
      (repo: Repository) => {
        const checkSets = getCheckSets(repo);

        if (checkSets.length === 0) return 0;
        return checkSets.includes(CheckSet.Community) ? -1 : 1;
      },
      'score.global',
      'name',
    ],
    ['asc', 'desc', 'asc']
  );
};

export default sortRepos;
