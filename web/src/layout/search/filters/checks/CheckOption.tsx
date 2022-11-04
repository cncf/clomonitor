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
      <div className={`btn-group me-2 me-sm-4 ${styles.btns}`} role="group">
        <div className="d-none d-md-flex">
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
                aria-label={`${isInPassingCheck ? 'Unselect' : 'Select'} ${option.name} as passed`}
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
                aria-label={`${isInNotPassingCheck ? 'Unselect' : 'Select'} ${option.name} as not passed`}
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
        <div className="d-flex d-md-none">
          <button
            type="button"
            className={classNames('btn rounded-0 border p-0', styles.btn, styles.passingBtn, {
              [styles.isPassing]: isInPassingCheck,
            })}
            onClick={() => {
              props.onChange(FilterKind.PassingCheck, props.option, !isInPassingCheck);
            }}
            aria-label={`${isInPassingCheck ? 'Unselect' : 'Select'} ${option.name} as passed`}
          >
            <div className="d-flex align-items-center justify-content-center">
              <GoCheck />
            </div>
          </button>
          <button
            type="button"
            className={classNames('btn rounded-0 border p-0', styles.btn, {
              [styles.isNotPassing]: isInNotPassingCheck,
            })}
            onClick={() => {
              props.onChange(FilterKind.NotPassingCheck, props.option, !isInNotPassingCheck);
            }}
            aria-label={`${isInNotPassingCheck ? 'Unselect' : 'Select'} ${option.name} as not passed`}
          >
            <div className="d-flex align-items-center justify-content-center">
              <GoX />
            </div>
          </button>
        </div>
      </div>
      <small className="me-1 me-md-2 position-relative">{option.icon}</small>
      <div className={`me-0 me-sm-2 text-truncate flex-grow-1 ${styles.label}`}>
        <span className="d-none d-sm-block text-truncate">{option.name}</span>
        <span className="d-block d-sm-none text-truncate">{option.shortName || option.name}</span>
      </div>
    </div>
  );
};

export default CheckOption;
