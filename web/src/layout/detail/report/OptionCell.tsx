import { isUndefined } from 'lodash';
import { FaRegCheckCircle, FaRegTimesCircle } from 'react-icons/fa';
import { FiExternalLink } from 'react-icons/fi';

import { REPORT_OPTIONS } from '../../../data';
import { ReportCheck, ReportOption, ReportOptionData } from '../../../types';
import ExternalLink from '../../common/ExternalLink';
import styles from './OptionCell.module.css';

interface Props {
  label: ReportOption;
  check: ReportCheck;
}

function getOptionInfo(key: ReportOption) {
  return REPORT_OPTIONS[key];
}

const OptionCell = (props: Props) => {
  const errorIcon = <FaRegTimesCircle data-testid="error-icon" className={`text-danger ${styles.icon}`} />;
  const successIcon = <FaRegCheckCircle data-testid="success-icon" className={`text-success ${styles.icon}`} />;

  const opt: ReportOptionData = getOptionInfo(props.label);

  const getCheckValue = (): string => {
    switch (props.label) {
      case ReportOption.SPDX:
        return props.check.value || 'Not detected';

      default:
        return opt.name;
    }
  };

  return (
    <tr>
      <td className={`text-center ${styles.iconCell}`}>{props.check.passed ? successIcon : errorIcon}</td>
      <td>
        <div className={`d-table w-100 ${styles.contentCell}`}>
          <div className="d-flex flex-row align-items-baseline align-items-lg-center">
            <div className="text-muted me-2">{opt.icon}</div>
            <div className="d-flex flex-column align-items-start flex-grow-1 truncateWrapper">
              <div data-testid="opt-name" className={`d-flex flex-row align-items-center w-100 ${styles.name}`}>
                {!isUndefined(props.check.url) ? (
                  <ExternalLink href={props.check.url} className="d-inline w-100">
                    <div className="d-flex flex-row align-items-center w-100">
                      <small className="fw-bold text-truncate">{getCheckValue()}</small>
                      <FiExternalLink className={`ms-2 ${styles.miniIcon}`} />
                    </div>
                  </ExternalLink>
                ) : (
                  <small className="fw-bold text-truncate">{getCheckValue()}</small>
                )}
              </div>
              <div className={`d-none d-lg-block text-muted text-truncate w-100 ${styles.legend}`}>{opt.legend}</div>
            </div>
          </div>
        </div>
      </td>
    </tr>
  );
};

export default OptionCell;
