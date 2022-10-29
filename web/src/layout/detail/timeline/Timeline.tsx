import classNames from 'classnames';
import { isUndefined, uniq } from 'lodash';
import moment from 'moment';
import { Dispatch, Fragment, SetStateAction, useEffect, useState } from 'react';
import { RiHistoryFill } from 'react-icons/ri';

import { SortedDates } from '../../../types';
import groupDatesByYearAndMonth from '../../../utils/groupDatesByYearAndMonth';
import styles from './Timeline.module.css';

interface Props {
  snapshots: string[];
  activeDate?: string;
  currentSearch?: string;
  setActiveDate: Dispatch<SetStateAction<string | undefined>>;
}

const MIN_NUMBER_SNAPSHOTS = 3;

const Timeline = (props: Props) => {
  const [today] = useState<string>(moment().format('YYYY-MM-DD'));
  const [availablaSnapshots, setAvailablaSnapshots] = useState<string[]>([]);
  const [dates, setDates] = useState<SortedDates | undefined>();

  useEffect(() => {
    const formatDates = () => {
      const allDates = uniq([today, ...props.snapshots]);
      setAvailablaSnapshots(allDates);
      setDates(groupDatesByYearAndMonth(allDates));
    };
    formatDates();
  }, [props.snapshots]); /* eslint-disable-line react-hooks/exhaustive-deps */

  if (isUndefined(dates) || availablaSnapshots.length < MIN_NUMBER_SNAPSHOTS) return null;

  return (
    <div className="d-none d-sm-flex my-4 my-md-5 ms-4 ms-md-5">
      <div>
        <div className="d-flex flex-column text-center border">
          <div className="mt-2 pb-3 fs-4">
            <RiHistoryFill />
          </div>
          <div className="d-flex flex-column text-center">
            {Object.keys(dates)
              .sort()
              .reverse()
              .map((year: string) => {
                return (
                  <div className="mb-3" key={`year_${year}`}>
                    <div className={`fw-bold w-100 border-top border-bottom px-1 mb-3 ${styles.year}`}>{year}</div>
                    {Object.keys(dates[year])
                      .sort()
                      .reverse()
                      .map((month: string) => {
                        return (
                          <Fragment key={`date_${year}_${month}`}>
                            <div className={`position-relative mt-3 mb-2 fw-bold text-uppercase ${styles.month}`}>
                              {moment.monthsShort(parseInt(month) - 1)}
                            </div>
                            <div className="d-flex flex-column align-items-center">
                              {dates[year][month]
                                .sort()
                                .reverse()
                                .map((time: string) => {
                                  const isToday = time === today;
                                  const selectedDate = props.activeDate || today;
                                  const isActive = selectedDate === time;

                                  return (
                                    <button
                                      key={`time_${time}`}
                                      className={classNames(
                                        'position-relative btn btn-link text-decoration-none text-center text-muted rounded-circle my-1 p-0',
                                        styles.dot,
                                        {
                                          [styles.activeDot]: isActive,
                                        }
                                      )}
                                      onClick={() => {
                                        props.setActiveDate(!isToday ? time : undefined);
                                      }}
                                      disabled={isActive}
                                      aria-label={`Opens snapshot: ${moment(time, 'YYYY-MM-DD').format("Do MMM 'YY")}`}
                                    >
                                      <div className={styles.content}>{moment(time, 'YYYY-MM-DD').format('D')}</div>
                                    </button>
                                  );
                                })}
                            </div>
                          </Fragment>
                        );
                      })}
                  </div>
                );
              })}
          </div>
        </div>
      </div>
    </div>
  );
};

export default Timeline;
