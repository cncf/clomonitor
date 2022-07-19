import { isUndefined } from 'lodash';
import { BsDot } from 'react-icons/bs';

import { CATEGORY_NAMES, CHECKS_PER_CATEGORY } from '../../../../data';
import { FilterKind, ReportOption, ScoreType } from '../../../../types';
import styles from './Block.module.css';
import CheckOption from './CheckOption';

interface Props {
  type: ScoreType;
  activePassingChecks: ReportOption[];
  activeNotPassingChecks: ReportOption[];
  onChange: (name: FilterKind.PassingCheck | FilterKind.NotPassingCheck, value: ReportOption, checked: boolean) => void;
  resetChecksPerCategory: (category: ScoreType) => void;
  markAllAsPassedPerCategory: (category: ScoreType) => void;
  markAllAsNotPassedPerCategory: (category: ScoreType) => void;
}

const Block = (props: Props) => {
  const checks = CHECKS_PER_CATEGORY[props.type];
  if (isUndefined(checks)) return null;

  return (
    <div className="mb-4">
      <div className="d-flex flex-column mb-3">
        <small className="fw-bold text-uppercase text-muted">{CATEGORY_NAMES[props.type]}</small>
        <div className="d-flex flex-row align-items-center mt-1">
          <button
            className={`btn btn-link btn-sm p-0 ${styles.btn}`}
            onClick={() => props.markAllAsPassedPerCategory(props.type)}
          >
            All passed
          </button>
          <BsDot className="mx-1" />
          <button
            className={`btn btn-link btn-sm p-0 ${styles.btn}`}
            onClick={() => props.markAllAsNotPassedPerCategory(props.type)}
          >
            None passed
          </button>
          <BsDot className="mx-1" />
          <button
            className={`btn btn-link btn-sm p-0 ${styles.btn}`}
            onClick={() => props.resetChecksPerCategory(props.type)}
          >
            Reset
          </button>
        </div>
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
