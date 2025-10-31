import 'react-date-range/dist/styles.css';
import 'react-date-range/dist/theme/default.css';

import classNames from 'classnames';
import { useOutsideClick } from 'clo-ui/hooks/useOutsideClick';
import { format, parseISO } from 'date-fns';
import { useRef, useState } from 'react';
import { Calendar, type Range, type RangeKeyDict } from 'react-date-range';

import styles from './DateRangeBtn.module.css';

export enum DateRange {
  From = 'from',
  To = 'to',
}

const LABELS: Record<DateRange, string> = {
  [DateRange.From]: 'From',
  [DateRange.To]: 'To',
};

export interface DateRangeBtnProps {
  date: string;
  min: string;
  max: string;
  type: DateRange;
  onDateChange: (value: string, type: DateRange) => void;
}

const DateRangeBtn = (props: DateRangeBtnProps) => {
  const dropdownRef = useRef<HTMLDivElement | null>(null);
  const buttonRef = useRef<HTMLButtonElement | null>(null);
  const [isOpen, setIsOpen] = useState(false);

  useOutsideClick([dropdownRef, buttonRef], isOpen, () => setIsOpen(false));

  const onCalendarChange = (value: Date | RangeKeyDict) => {
    const nextDate = value instanceof Date ? value : getFirstDate(value);
    if (!nextDate) {
      return;
    }
    props.onDateChange(format(nextDate, 'yyyy-MM-dd'), props.type);
    setIsOpen(false);
  };

  return (
    <div className="position-relative d-inline-block">
      <div className={`text-uppercase text-muted mb-1 ${styles.legend}`}>{LABELS[props.type]}:</div>
      <button
        ref={buttonRef}
        className={`btn btn-sm btn-outline-secondary rounded-0 ${styles.dateBtn}`}
        onClick={() => setIsOpen(!isOpen)}
        aria-label={`Open calendar to choose date ${props.type}`}
        type="button"
      >
        {format(parseISO(props.date), 'MMM d, yyyy')}
      </button>
      <div
        role="complementary"
        ref={dropdownRef}
        className={classNames('dropdown-menu tooltipDropdown rounded-0 text-wrap mt-2 p-0', {
          show: isOpen,
        })}
      >
        <div className={`${styles.arrow} arrow`} />
        <Calendar
          onChange={onCalendarChange}
          minDate={parseISO(props.min)}
          maxDate={parseISO(props.max)}
          date={parseISO(props.date)}
          className={styles.calendar}
        />
      </div>
    </div>
  );
};

export default DateRangeBtn;

const getFirstDate = (ranges: RangeKeyDict): Date | undefined => {
  const [firstKey] = Object.keys(ranges);
  if (!firstKey) {
    return undefined;
  }
  const range: Range | undefined = ranges[firstKey];
  return range?.startDate ?? undefined;
};
