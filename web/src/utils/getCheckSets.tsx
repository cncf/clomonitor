import { CheckSet } from 'clo-ui/components/CheckSetBadge';

import { Repository } from '../types';

const getCheckSets = (repo: Repository): CheckSet[] => {
  if (repo.report && repo.report.check_sets) {
    return repo.report.check_sets;
  }
  return repo.check_sets || [];
};

export default getCheckSets;
