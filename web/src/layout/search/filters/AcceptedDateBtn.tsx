import 'react-date-range/dist/styles.css'; // main style file
import 'react-date-range/dist/theme/default.css'; // theme css file

import classNames from 'classnames';
import moment from 'moment';
import { useRef, useState } from 'react';
import { Calendar } from 'react-date-range';

import useOutsideClick from '../../../hooks/useOutsideClick';
import { AcceptedRangeKind } from '../../../types';
import styles from './AcceptedDateBtn.module.css';

interface Props {
  date: string;
  min: string;
  max: string;
  type: AcceptedRangeKind;
  onDateChange: (date: string, type: AcceptedRangeKind) => void;
}

const LEGEND = {
  [AcceptedRangeKind.From]: 'From',
  [AcceptedRangeKind.To]: 'To',
};

const AcceptedDateBtn = (props: Props) => {
  const ref = useRef<HTMLDivElement>(null);
  const [showCalendar, setShowCalendar] = useState<boolean>(false);
  useOutsideClick([ref], showCalendar, () => setShowCalendar(false));

  const handleChange = (date: Date) => {
    props.onDateChange(moment(date).format('YYYY-MM-DD'), props.type);
    setShowCalendar(false);
  };

  return (
    <>
      <div className={`text-uppercase text-muted mb-1 ${styles.legend}`}>{LEGEND[props.type]}:</div>
      <button
        className={`btn btn-sm btn-outline-secondary rounded-0 ${styles.dateBtn}`}
        onClick={() => setShowCalendar(!showCalendar)}
        aria-label={`Open calendar to choose date ${props.type}`}
      >
        {moment(props.date).format('MMM D, YYYY')}
      </button>

      <div
        role="complementary"
        ref={ref}
        className={classNames(styles.dropdown, 'dropdown-menu tooltipDropdown rounded-0 text-wrap mt-2 p-0', {
          show: showCalendar,
        })}
      >
        <div className={`arrow ${styles.arrow}`} />

        <Calendar
          onChange={handleChange}
          minDate={moment(props.min).toDate()}
          maxDate={moment(props.max).toDate()}
          date={moment(props.date).toDate()}
          className={styles.calendar}
        />
      </div>
    </>
  );
};

export default AcceptedDateBtn;
