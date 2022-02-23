import { isBoolean, isNull, isUndefined } from 'lodash';
import { FaRegCheckCircle, FaRegQuestionCircle, FaRegTimesCircle } from 'react-icons/fa';

import { REPORT_OPTIONS } from '../../../data';
import { ReportOption, ReportOptionData } from '../../../types';
import ElementWithTooltip from '../../common/ElementWithTooltip';
import styles from './OptionCell.module.css';

interface Props {
  label: ReportOption;
  value: boolean | string;
}

function getOptionInfo(key: ReportOption) {
  return REPORT_OPTIONS[key];
}

const OptionCell = (props: Props) => {
  const errorIcon = <FaRegTimesCircle data-testid="error-icon" className={`text-danger ${styles.icon}`} />;
  const successIcon = <FaRegCheckCircle data-testid="success-icon" className={`text-success ${styles.icon}`} />;

  const opt: ReportOptionData = getOptionInfo(props.label);

  const checkStatus = (): boolean => {
    if (isBoolean(props.value)) {
      if (props.value) {
        return true;
      } else {
        return false;
      }
    } else {
      if (isUndefined(props.value) || isNull(props.value) || props.value === '') {
        return false;
      } else {
        return true;
      }
    }
  };

  const status: boolean = checkStatus();

  return (
    <tr>
      <td className={`text-center ${styles.iconCell}`}>{status ? successIcon : errorIcon}</td>
      <td>
        <div className={`d-table w-100 ${styles.contentCell}`}>
          <div className="d-flex flex-row align-items-baseline align-items-lg-center">
            <div className="text-muted me-2">{opt.icon}</div>
            <div className="d-flex flex-column align-items-start flex-grow-1 truncateWrapper">
              <div data-testid="opt-name" className={`d-flex flex-row align-items-center mt-1 ${styles.name}`}>
                {(() => {
                  switch (props.label) {
                    case ReportOption.SPDX:
                      return <small className="fw-bold">{props.value || 'Not detected'}</small>;

                    default:
                      return <small className="fw-bold">{opt.name}</small>;
                  }
                })()}
              </div>
              <div className={`d-none d-lg-block text-muted text-truncate mt-1 w-100 ${styles.legend}`}>
                {opt.legend}
              </div>
            </div>
          </div>
        </div>
      </td>
      <td className={`text-center text-muted ${styles.iconCell}`}>
        <ElementWithTooltip
          className="ms-2 lh-1"
          element={<FaRegQuestionCircle className={styles.icon} />}
          tooltipWidth={285}
          tooltipClassName={styles.tooltipMessage}
          tooltipMessage={<div className="my-2">{opt.description}</div>}
          visibleTooltip
          active
        />
      </td>
    </tr>
  );
};

export default OptionCell;
