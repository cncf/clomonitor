import { format } from 'date-fns';
import isEmpty from 'lodash/isEmpty';
import isEqual from 'lodash/isEqual';
import isNil from 'lodash/isNil';
import omitBy from 'lodash/omitBy';
import { useEffect, useId, useMemo, useState } from 'react';

import DateRangeBtn, { DateRange } from './DateRangeBtn';
import styles from './DateRangeFilter.module.css';

export interface DateRangeOpts {
  from?: string;
  to?: string;
}

interface Props {
  initialDate: string;
  from?: string;
  to?: string;
  onDateRangeChange: (range: DateRangeOpts) => void;
}

const DateRangeFilter = (props: Props) => {
  const today = useMemo(() => format(new Date(), 'yyyy-MM-dd'), []);
  const [fromDate, setFromDate] = useState(props.initialDate);
  const [toDate, setToDate] = useState(today);

  const generatedId = useId().replace(/:/g, '');
  const fromInputId = `${generatedId}-from`;
  const toInputId = `${generatedId}-to`;

  const onDateChange = (value: string, type: DateRange) => {
    if (type === DateRange.From) {
      setFromDate(value);
      return;
    }
    setToDate(value);
  };

  useEffect(() => {
    setFromDate(props.from ?? props.initialDate);
    setToDate(props.to ?? today);
  }, [props.from, props.to, props.initialDate, today]);

  useEffect(() => {
    const timeout = setTimeout(() => {
      const nextRange: DateRangeOpts = {};
      if (toDate !== today) {
        nextRange.to = toDate;
      }
      if (fromDate !== props.initialDate) {
        nextRange.from = fromDate;
      }
      const currentRange = omitBy({ from: props.from, to: props.to }, isNil);
      if (isEmpty(nextRange) || isEqual(nextRange, currentRange)) {
        return;
      }
      props.onDateRangeChange(nextRange);
    }, 300);

    return () => {
      clearTimeout(timeout);
    };
  }, [fromDate, toDate, props.from, props.to, props.initialDate, props.onDateRangeChange, today]);

  return (
    <>
      <div className={`fw-bold text-uppercase text-primary ${styles.categoryTitle}`}>
        <small>Accepted</small>
      </div>

      <div className={`flex-column mb-4 ${styles.mobileDateRange}`}>
        <div className="my-2">
          <label htmlFor={fromInputId} className={`form-label text-uppercase text-muted ${styles.label}`}>
            From:
          </label>
          <input
            type="date"
            className={`form-control form-control-sm rounded-0 ${styles.input}`}
            min={props.initialDate}
            max={toDate}
            id={fromInputId}
            value={fromDate}
            onChange={(event) => onDateChange(event.target.value, DateRange.From)}
          />
        </div>

        <div>
          <label htmlFor={toInputId} className={`form-label text-uppercase text-muted ${styles.label}`}>
            To:
          </label>
          <input
            type="date"
            className={`form-control form-control-sm rounded-0 ${styles.input}`}
            min={fromDate}
            max={today}
            id={toInputId}
            value={toDate}
            onChange={(event) => onDateChange(event.target.value, DateRange.To)}
          />
        </div>
      </div>

      <div className={`mt-3 mb-4 ${styles.desktopDateRange}`}>
        <DateRangeBtn
          date={fromDate}
          min={props.initialDate}
          max={toDate}
          onDateChange={onDateChange}
          type={DateRange.From}
        />
        <div className="mt-3">
          <DateRangeBtn date={toDate} min={fromDate} max={today} onDateChange={onDateChange} type={DateRange.To} />
        </div>
      </div>
    </>
  );
};

export default DateRangeFilter;
