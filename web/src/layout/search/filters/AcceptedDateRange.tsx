import 'react-date-range/dist/styles.css'; // main style file
import 'react-date-range/dist/theme/default.css'; // theme css file

import classNames from 'classnames';
import moment from 'moment';
import { useEffect, useRef, useState } from 'react';
import { DateRange, Range, RangeKeyDict } from 'react-date-range';

import useOutsideClick from '../../../hooks/useOutsideClick';
import styles from './AcceptedDateRange.module.css';

interface Props {
  acceptedFrom?: string;
  acceptedTo?: string;
  onAcceptedDateRangeChange: (dates: AcceptedDate) => void;
}

interface AcceptedDate {
  accepted_from?: string;
  accepted_to?: string;
}

const INITIAL_DATE = '2016-01-01';

const AcceptedDateRange = (props: Props) => {
  const ref = useRef<HTMLDivElement>(null);
  const [showCalendar, setShowCalendar] = useState<boolean>(false);
  useOutsideClick([ref], showCalendar, () => setShowCalendar(false));

  const [ranges, setRanges] = useState<Range[]>([
    {
      startDate: moment(INITIAL_DATE).toDate(),
      endDate: moment().toDate(),
      key: 'selection',
    },
  ]);

  const handleChange = (data: RangeKeyDict) => {
    setRanges([data.selection]);
    props.onAcceptedDateRangeChange({
      accepted_from: moment(data.selection.startDate!).format('yyyy-MM-DD'),
      accepted_to: moment(data.selection.endDate!).format('yyyy-MM-DD'),
    });
  };

  useEffect(() => {
    setRanges([
      {
        startDate: moment(props.acceptedFrom || INITIAL_DATE).toDate(),
        endDate: moment(props.acceptedTo || moment()).toDate(),
        key: 'selection',
      },
    ]);
  }, [props.acceptedFrom, props.acceptedTo]); /* eslint-disable-line react-hooks/exhaustive-deps */

  return (
    <>
      <div className={`fw-bold text-uppercase text-primary ${styles.categoryTitle}`}>
        <small>Accepted</small>
      </div>

      <div className="mt-3">
        <button
          className={`btn btn-sm btn-outline-secondary rounded-0 w-100 ${styles.dateBtn}`}
          onClick={() => setShowCalendar(!showCalendar)}
          aria-label="Open calendar"
        >
          {moment(ranges[0].startDate!).format('MMM D, yyyy')} - {moment(ranges[0].endDate!).format('MMM D, yyyy')}
        </button>

        <div
          role="complementary"
          ref={ref}
          className={classNames(styles.dropdown, 'dropdown-menu tooltipDropdown rounded-0 text-wrap mt-2 p-0', {
            show: showCalendar,
          })}
        >
          <div className={`arrow ${styles.arrow}`} />

          <div className={`d-flex flex-row w-100 pt-3 text-uppercase text-muted ${styles.legends}`}>
            <div className={`w-50 ${styles.legendFrom}`}>From:</div>
            <div className={`w-50 ${styles.legendTo}`}>To:</div>
          </div>

          <DateRange
            className={styles.dateRange}
            editableDateInputs={true}
            onChange={handleChange}
            moveRangeOnFirstSelection={false}
            ranges={ranges}
            minDate={moment(INITIAL_DATE).toDate()}
            maxDate={moment().toDate()}
          />
        </div>
      </div>
    </>
  );
};

export default AcceptedDateRange;
