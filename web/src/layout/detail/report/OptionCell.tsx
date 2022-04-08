import { isUndefined } from 'lodash';
import { FaRegCheckCircle, FaRegTimesCircle } from 'react-icons/fa';
import { FiExternalLink } from 'react-icons/fi';
import { MdRemoveCircleOutline } from 'react-icons/md';
import { RiErrorWarningLine } from 'react-icons/ri';

import { REPORT_OPTIONS } from '../../../data';
import { ReportCheck, ReportOption, ReportOptionData } from '../../../types';
import ElementWithTooltip from '../../common/ElementWithTooltip';
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
  const exemptIcon = <MdRemoveCircleOutline data-testid="exempt-icon" className={`text-muted ${styles.exemptIcon}`} />;
  const failedIcon = <RiErrorWarningLine data-testid="failed-icon" className={styles.failedIcon} />;

  const opt: ReportOptionData = getOptionInfo(props.label);

  const getCheckValue = (): string => {
    switch (props.label) {
      case ReportOption.SPDX:
        return props.check.value || 'Not detected';

      default:
        return opt.name;
    }
  };

  const getIconCheck = (): JSX.Element => {
    if (!isUndefined(props.check.exempt) && props.check.exempt) {
      return (
        <>
          {!isUndefined(props.check.exemption_reason) && props.check.exemption_reason !== '' ? (
            <>
              <ElementWithTooltip
                element={exemptIcon}
                tooltipWidth={500}
                className="cursorPointer"
                tooltipClassName={styles.reasonTooltipMessage}
                tooltipMessage={
                  <div className="text-start p-2">
                    <div className="border-bottom pb-2 mb-3 fw-bold">
                      This repository is exempt from passing this check
                    </div>
                    <div className={`text-break ${styles.reason}`}>
                      <span className="fw-bold">Reason:</span> {props.check.exemption_reason}
                    </div>
                  </div>
                }
                alignmentTooltip="left"
                forceAlignment
                visibleTooltip
                active
              />
              <span className="d-block d-md-none">{exemptIcon}</span>
            </>
          ) : (
            <>{exemptIcon}</>
          )}
        </>
      );
    } else if (!isUndefined(props.check.failed) && props.check.failed) {
      return (
        <>
          {!isUndefined(props.check.fail_reason) && props.check.fail_reason !== '' ? (
            <>
              <ElementWithTooltip
                element={failedIcon}
                tooltipWidth={500}
                className="cursorPointer"
                tooltipClassName={styles.reasonTooltipMessage}
                tooltipMessage={
                  <div className="text-start p-2">
                    <div className="border-bottom pb-2 mb-3 fw-bold">Something went wrong running this check</div>
                    <div className={`text-truncate ${styles.reason}`}>
                      <span className="fw-bold">Reason:</span> {props.check.fail_reason}
                    </div>
                  </div>
                }
                alignmentTooltip="left"
                forceAlignment
                visibleTooltip
                active
              />
              <span className="d-block d-md-none">{failedIcon}</span>
            </>
          ) : (
            <>{failedIcon}</>
          )}
        </>
      );
    } else {
      return props.check.passed ? successIcon : errorIcon;
    }
  };

  return (
    <tr>
      <td className={`text-center ${styles.iconCell}`}>{getIconCheck()}</td>
      <td>
        <div className={`d-table w-100 ${styles.contentCell}`}>
          <div className="d-flex flex-row align-items-baseline align-items-lg-center">
            <div className="text-muted me-2">{opt.icon}</div>
            <div className="d-flex flex-column align-items-start flex-grow-1 truncateWrapper">
              <div data-testid="opt-name" className={`d-flex flex-row align-items-center w-100 ${styles.name}`}>
                {!isUndefined(props.check.url) ? (
                  <div>
                    <ExternalLink href={props.check.url} className="d-inline w-100">
                      <div className="d-flex flex-row align-items-center w-100">
                        <small className="fw-bold text-truncate">{getCheckValue()}</small>
                        <FiExternalLink className={`ms-2 ${styles.miniIcon}`} />
                      </div>
                    </ExternalLink>
                  </div>
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
