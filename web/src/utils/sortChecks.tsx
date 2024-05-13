import { isUndefined } from 'lodash';

import { ReportOption } from '../types';

const sortChecks = (opts?: object): ReportOption[] => {
  if (isUndefined(opts)) return [];
  const optNames: ReportOption[] = [];
  Object.keys(opts).forEach((opt: string) => {
    // we check that opt belongs to ReportOption enum
    if (Object.values(ReportOption).includes(opt as ReportOption)) {
      optNames.push(opt as ReportOption);
    }
  });

  const sortedNames = optNames.sort((a, b) => {
    // spdxId is always first item in its category
    if (a === ReportOption.SPDX || b === ReportOption.SPDX) return -1;
    const nameA = a.toLowerCase();
    const nameB = b.toLowerCase();
    if (nameA < nameB) return -1;
    if (nameA > nameB) return 1;
    return 0;
  });

  return sortedNames;
};

export default sortChecks;
