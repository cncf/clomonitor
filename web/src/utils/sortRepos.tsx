import { isUndefined, orderBy } from 'lodash';

import { CheckSet, Repository } from '../types';

// Sort by community repo kind, score global and alphabetically
const sortRepos = (repos: Repository[]): Repository[] => {
  return orderBy(
    repos,
    [
      (repo: Repository) => {
        if (isUndefined(repo.check_sets)) return 0;
        return repo.check_sets.includes(CheckSet.Community) ? -1 : 1;
      },
      'score.global',
      'name',
    ],
    ['asc', 'desc', 'asc']
  );
};

export default sortRepos;
