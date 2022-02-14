import { isBoolean, isNull, isUndefined } from 'lodash';
import { FaRegCheckCircle, FaRegTimesCircle } from 'react-icons/fa';

import { REPORT_OPTIONS } from '../../../data';
import { ReportOption, ReportOptionData } from '../../../types';
import styles from './OptionCell.module.css';

interface Props {
  label: ReportOption;
  value: boolean | string;
}

function getOptionInfo(key: ReportOption) {
  return REPORT_OPTIONS[key];
}

const OptionCell = (props: Props) => {
  const errorIcon = <FaRegTimesCircle className={`text-danger ${styles.icon}`} />;
  const successIcon = <FaRegCheckCircle className={`text-success ${styles.icon}`} />;

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
      <td className="text-center">{status ? successIcon : errorIcon}</td>
      <td className="text-center">
        <div className="d-flex flex-row align-items-center">
          <div className="text-muted me-2">{opt.icon}</div>
          <div className={`d-flex flex-row align-items-center mt-1 ${styles.name}`}>
            {(() => {
              switch (props.label) {
                case ReportOption.SPDX:
                  return <small className="fw-bold">{props.value || 'Not detected'}</small>;

                default:
                  return <small className="fw-bold">{opt.name}</small>;
              }
            })()}
          </div>
        </div>
      </td>
      <td className={`fw-bold text-muted text-center ${styles.weight}`}>
        {opt.weight}
        <small>%</small>
      </td>
    </tr>
  );
};

export default OptionCell;
