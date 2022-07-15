import { isUndefined } from 'lodash';

import { CATEGORY_NAMES, CHECKS_PER_CATEGORY } from '../../../../data';
import { FilterKind, ReportOption, ScoreType } from '../../../../types';
import CheckOption from './CheckOption';

interface Props {
  type: ScoreType;
  activePassingChecks: ReportOption[];
  activeNotPassingChecks: ReportOption[];
  onChange: (name: FilterKind.PassingCheck | FilterKind.NotPassingCheck, value: ReportOption, checked: boolean) => void;
}

const Block = (props: Props) => {
  const checks = CHECKS_PER_CATEGORY[props.type];
  if (isUndefined(checks)) return null;

  return (
    <div className="mb-4">
      <div className="fw-bold text-uppercase text-muted mb-3">
        <small>{CATEGORY_NAMES[props.type]}</small>
      </div>
      {checks.map((check: ReportOption) => {
        return (
          <CheckOption
            type={props.type}
            option={check}
            activePassingChecks={props.activePassingChecks}
            activeNotPassingChecks={props.activeNotPassingChecks}
            onChange={props.onChange}
          />
        );
      })}
    </div>
  );
};

export default Block;
