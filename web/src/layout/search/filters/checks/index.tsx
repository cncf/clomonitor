import classNames from 'classnames';
import { isEmpty, isUndefined } from 'lodash';
import { useEffect, useState } from 'react';
import { BsCheckAll } from 'react-icons/bs';
import { GoCheck, GoX } from 'react-icons/go';
import { IoMdCloseCircleOutline } from 'react-icons/io';

import { CATEGORY_NAMES, CHECKS_PER_CATEGORY, REPORT_OPTIONS } from '../../../../data';
import { FilterKind, ReportOption, ScoreType } from '../../../../types';
import ElementWithTooltip from '../../../common/ElementWithTooltip';
import Modal from '../../../common/Modal';
import Block from './Block';
import styles from './Checks.module.css';

interface Props {
  activePassingChecks?: ReportOption[];
  activeNotPassingChecks?: ReportOption[];
  onChecksChange: (filters: FiltersProp) => void;
  onChange: (name: string, value: string, checked: boolean) => void;
}

interface Check {
  option: ReportOption;
  name: string;
  shortName?: string;
  passing: boolean;
}

type Checks = {
  [key: string]: Check[];
};

interface FiltersProp {
  [key: string]: string[];
}

const ChecksFilter = (props: Props) => {
  const [openStatus, setOpenStatus] = useState(false);
  const [checks, setChecks] = useState<JSX.Element | null>(null);
  const [selectedChecks, setSelectedChecks] = useState({
    [FilterKind.PassingCheck]: props.activePassingChecks || [],
    [FilterKind.NotPassingCheck]: props.activeNotPassingChecks || [],
  });

  const noActiveChecks =
    (isUndefined(props.activePassingChecks) || props.activePassingChecks.length === 0) &&
    (isUndefined(props.activeNotPassingChecks) || props.activeNotPassingChecks.length === 0);

  const onCloseModal = () => {
    props.onChecksChange(selectedChecks);
    setOpenStatus(false);
  };

  const resetChecks = () => {
    setSelectedChecks({
      [FilterKind.PassingCheck]: [],
      [FilterKind.NotPassingCheck]: [],
    });
  };

  const onCheckChange = (
    name: FilterKind.PassingCheck | FilterKind.NotPassingCheck,
    value: ReportOption,
    checked: boolean
  ): void => {
    let newFilters = selectedChecks[name].slice();
    let moreFilters = {};
    if (checked) {
      newFilters.push(value);
      let oppositeName = name === FilterKind.PassingCheck ? FilterKind.NotPassingCheck : FilterKind.PassingCheck;
      moreFilters = { [oppositeName]: selectedChecks[oppositeName].filter((el) => el !== value) };
    } else {
      newFilters = newFilters.filter((el) => el !== value);
    }

    setSelectedChecks({
      ...selectedChecks,
      [name]: newFilters,
      ...moreFilters,
    });
  };

  useEffect(() => {
    const getSelectedChecks = (): JSX.Element | null => {
      const tmpChecks: Checks = {};

      for (let cat of Object.keys(CHECKS_PER_CATEGORY)) {
        (CHECKS_PER_CATEGORY[cat as ScoreType] as ReportOption[]).forEach((opt: ReportOption) => {
          const isInPassingCheck = props.activePassingChecks ? props.activePassingChecks.includes(opt) : false;
          const isInNotPassingCheck = props.activeNotPassingChecks ? props.activeNotPassingChecks.includes(opt) : false;
          if (isInPassingCheck || isInNotPassingCheck) {
            const option: Check = {
              option: opt,
              name: REPORT_OPTIONS[opt].name,
              shortName: REPORT_OPTIONS[opt].shortName,
              passing: isInPassingCheck,
            };
            if (isUndefined(tmpChecks[cat])) {
              tmpChecks[cat] = [option];
            } else {
              tmpChecks[cat] = [...tmpChecks[cat], option];
            }
          }
        });
      }

      if (isEmpty(tmpChecks)) return null;

      return (
        <div className={styles.wrapperChecks}>
          {Object.keys(tmpChecks).map((cat: string) => {
            return (
              <div key={`checked_${cat}`} className="mb-3">
                <div className={`text-uppercase text-muted fw-bold mb-2 ${styles.subtitle}`}>
                  {CATEGORY_NAMES[cat as ScoreType]}
                </div>
                {tmpChecks[cat].map((opt: Check) => {
                  return (
                    <div key={`checked_${cat}_${opt.name}`} className="d-flex align-items-center">
                      <ElementWithTooltip
                        element={
                          <>
                            {opt.passing ? (
                              <div
                                className={`d-flex align-items-center justify-content-center ${styles.squareCheck} ${styles.passing}`}
                              >
                                <GoCheck />
                              </div>
                            ) : (
                              <div
                                className={`d-flex align-items-center justify-content-center ${styles.squareCheck} ${styles.notPassing}`}
                              >
                                <GoX />
                              </div>
                            )}
                          </>
                        }
                        tooltipMessage={opt.passing ? 'Check passed' : 'Check not passed'}
                        tooltipArrowClassName={styles.iconTooltipArrow}
                        alignmentTooltip="left"
                        forceAlignment
                        visibleTooltip
                        active
                      />

                      <div className={`d-flex flex-row align-items-center ${styles.checkWrapper}`}>
                        <div className={`ms-2 text-truncate ${styles.checkName}`}>{opt.shortName || opt.name}</div>
                        <button
                          className={`btn btn-link text-decoration-none py-0 px-2 position-relative ${styles.btnCheck}`}
                          onClick={() => {
                            props.onChange(
                              opt.passing ? FilterKind.PassingCheck : FilterKind.NotPassingCheck,
                              opt.option,
                              false
                            );
                          }}
                        >
                          <IoMdCloseCircleOutline />
                        </button>
                      </div>
                    </div>
                  );
                })}
              </div>
            );
          })}
        </div>
      );
    };

    setSelectedChecks({
      [FilterKind.PassingCheck]: props.activePassingChecks || [],
      [FilterKind.NotPassingCheck]: props.activeNotPassingChecks || [],
    });
    setChecks(getSelectedChecks());
  }, [props.activePassingChecks, props.activeNotPassingChecks]); /* eslint-disable-line react-hooks/exhaustive-deps */

  return (
    <>
      <div className={`d-flex flex-row align-items-center text-primary ${styles.categoryTitle}`}>
        <div className="fw-bold text-uppercase">
          <small>Checks</small>
        </div>
      </div>

      {checks}

      <div>
        <button
          type="button"
          className={classNames('btn btn-outline-secondary btn-sm rounded-0 mb-4', styles.btn, {
            'mt-3': noActiveChecks,
          })}
          onClick={() => setOpenStatus(true)}
          aria-label="Open modal"
        >
          <span>{noActiveChecks ? 'Add' : 'Edit'} checks filters</span>
        </button>
      </div>

      {openStatus && (
        <Modal
          header="Checks filters"
          closeButton={
            <div className="d-flex flex-row align-items-center justify-content-between w-100">
              <button
                type="button"
                className="btn btn-sm rounded-0 btn-secondary text-uppercase"
                onClick={() => {
                  resetChecks();
                }}
                aria-label="Reset checks filters"
              >
                <div className="d-flex flex-row align-items-center">
                  <IoMdCloseCircleOutline className="me-2" />
                  <div>Reset </div>
                </div>
              </button>

              <button
                type="button"
                className="btn btn-sm rounded-0 btn-secondary text-uppercase"
                onClick={() => {
                  onCloseModal();
                }}
                aria-label="Close modal"
              >
                <div className="d-flex flex-row align-items-center">
                  <BsCheckAll className="me-2" />
                  <div>Apply</div>
                </div>
              </button>
            </div>
          }
          onClose={onCloseModal}
          size="md"
          open={openStatus}
        >
          <div className="w-100 position-relative">
            <div className={`text-muted ${styles.legend}`}>
              <p className="mb-3">
                Please note that checks are run per <span className="fst-italic">repository</span>, and this is a filter
                for <span className="fst-italic">projects</span>, which can have multiple repositories. We consider that
                a project passes a check if <span className="fst-italic">all repositories pass the check</span>.
                Similarly, we consider that a project does not pass a check if{' '}
                <span className="fst-italic">any of the repositories does not pass the check</span>.
              </p>
            </div>

            <Block
              type={ScoreType.Documentation}
              activePassingChecks={selectedChecks[FilterKind.PassingCheck]}
              activeNotPassingChecks={selectedChecks[FilterKind.NotPassingCheck]}
              onChange={onCheckChange}
            />
            <Block
              type={ScoreType.License}
              activePassingChecks={selectedChecks[FilterKind.PassingCheck]}
              activeNotPassingChecks={selectedChecks[FilterKind.NotPassingCheck]}
              onChange={onCheckChange}
            />
            <Block
              type={ScoreType.BestPractices}
              activePassingChecks={selectedChecks[FilterKind.PassingCheck]}
              activeNotPassingChecks={selectedChecks[FilterKind.NotPassingCheck]}
              onChange={onCheckChange}
            />
            <Block
              type={ScoreType.Security}
              activePassingChecks={selectedChecks[FilterKind.PassingCheck]}
              activeNotPassingChecks={selectedChecks[FilterKind.NotPassingCheck]}
              onChange={onCheckChange}
            />
            <Block
              type={ScoreType.Legal}
              onChange={onCheckChange}
              activePassingChecks={selectedChecks[FilterKind.PassingCheck]}
              activeNotPassingChecks={selectedChecks[FilterKind.NotPassingCheck]}
            />
          </div>
        </Modal>
      )}
    </>
  );
};

export default ChecksFilter;
