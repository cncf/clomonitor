import { isBoolean, isNull, isUndefined } from 'lodash';
import { FaRegCheckCircle, FaRegQuestionCircle, FaRegTimesCircle } from 'react-icons/fa';

import { REPORT_OPTIONS } from '../../../data';
import { ReportOption, ReportOptionData, RepositoryKind } from '../../../types';
import ElementWithTooltip from '../../common/ElementWithTooltip';
import styles from './OptionBox.module.css';

interface Props {
  repoKind: RepositoryKind;
  label: ReportOption;
  value: boolean | string;
}

function getOptionInfo(key: ReportOption) {
  return REPORT_OPTIONS[key];
}

const OptionBox = (props: Props) => {
  const errorIcon = <FaRegTimesCircle className="fs-5 text-danger" />;
  const successIcon = <FaRegCheckCircle className="fs-5 text-success" />;

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
    <div className={`p-3 ${styles.cardWrapper}`}>
      <div className={`border border-2 position-relative ${styles.card}`}>
        <div className="position-absolute top-0 start-0 end-0">
          <div className="d-flex flex-row justify-content-between pt-2 px-2 w-100">
            {status ? successIcon : errorIcon}

            <div>
              <div className={`fw-bold text-muted ${styles.weight}`}>
                {opt.weight[props.repoKind]}
                <small>%</small>
              </div>
            </div>
          </div>
        </div>
        <div className="d-flex flex-column align-items-center my-3">
          <div className="text-muted">{opt.icon}</div>
          <div className="d-flex flex-row align-items-center mt-1">
            {(() => {
              switch (props.label) {
                case ReportOption.SPDX:
                  return <small className="fw-bold">{props.value || 'Not detected'}</small>;

                default:
                  return <small className="fw-bold">{opt.name}</small>;
              }
            })()}
            <ElementWithTooltip
              className="ms-1 lh-1"
              element={<FaRegQuestionCircle />}
              tooltipWidth={285}
              tooltipClassName={styles.tooltipMessage}
              tooltipMessage={<div className="my-2">{opt.description}</div>}
              visibleTooltip
              active
            />
          </div>
        </div>
      </div>
    </div>
  );
};

export default OptionBox;
