import classNames from 'classnames';
import { isUndefined } from 'lodash';
import { GoCheck, GoX } from 'react-icons/go';

import { REPORT_OPTIONS } from '../../../../data';
import { FilterKind, ReportOption, ScoreType } from '../../../../types';
import ElementWithTooltip from '../../../common/ElementWithTooltip';
import styles from './CheckOption.module.css';

interface Props {
  type: ScoreType;
  option: ReportOption;
  activePassingChecks: ReportOption[];
  activeNotPassingChecks: ReportOption[];
  onChange: (name: FilterKind.PassingCheck | FilterKind.NotPassingCheck, value: ReportOption, checked: boolean) => void;
}

const CheckOption = (props: Props) => {
  const option = REPORT_OPTIONS[props.option];

  if (isUndefined(option)) return null;

  const isInPassingCheck = props.activePassingChecks.includes(props.option);
  const isInNotPassingCheck = props.activeNotPassingChecks.includes(props.option);

  return (
    <div className="d-flex flex-row align-items-center my-1">
      <div className={`btn-group me-4 ${styles.btns}`} role="group">
        <ElementWithTooltip
          element={
            <button
              type="button"
              className={classNames('btn rounded-0 border p-0', styles.btn, styles.passingBtn, {
                [styles.isPassing]: isInPassingCheck,
              })}
              onClick={() => {
                props.onChange(FilterKind.PassingCheck, props.option, !isInPassingCheck);
              }}
            >
              <div className="d-flex align-items-center justify-content-center">
                <GoCheck />
              </div>
            </button>
          }
          tooltipMessage="Check passed"
          tooltipArrowClassName={styles.iconTooltipArrow}
          alignmentTooltip="left"
          forceAlignment
          visibleTooltip
          delay={1200}
          active
          onlyInHoverableDevices
        />

        <ElementWithTooltip
          element={
            <button
              type="button"
              className={classNames('btn rounded-0 border p-0', styles.btn, {
                [styles.isNotPassing]: isInNotPassingCheck,
              })}
              onClick={() => {
                props.onChange(FilterKind.NotPassingCheck, props.option, !isInNotPassingCheck);
              }}
            >
              <div className="d-flex align-items-center justify-content-center">
                <GoX />
              </div>
            </button>
          }
          tooltipMessage="Check not passed"
          tooltipArrowClassName={styles.iconTooltipArrow}
          alignmentTooltip="left"
          forceAlignment
          visibleTooltip
          delay={1200}
          active
          onlyInHoverableDevices
        />
      </div>
      <small className="me-2">{option.icon}</small>
      <div className="me-2">{option.name}</div>
    </div>
  );
};

export default CheckOption;
