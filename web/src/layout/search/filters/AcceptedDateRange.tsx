import { isEmpty, isEqual, isNil, omitBy } from 'lodash';
import moment from 'moment';
import { ChangeEvent, useEffect, useRef, useState } from 'react';

import useOutsideClick from '../../../hooks/useOutsideClick';
import { AcceptedRangeKind } from '../../../types';
import AcceptedDateBtn from './AcceptedDateBtn';
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
  const END_DATE = moment().format('YYYY-MM-DD');
  const [dateFrom, setDateFrom] = useState<string>(INITIAL_DATE);
  const [dateTo, setDateTo] = useState<string>(END_DATE);
  const ref = useRef<HTMLDivElement>(null);
  const [showCalendar, setShowCalendar] = useState<boolean>(false);
  useOutsideClick([ref], showCalendar, () => setShowCalendar(false));

  const updateAcceptedRange = (dates: AcceptedDate) => {
    if (
      !isEmpty(dates) &&
      !isEqual(
        dates,
        omitBy(
          {
            accepted_from: props.acceptedFrom,
            accepted_to: props.acceptedTo,
          },
          isNil
        )
      )
    ) {
      props.onAcceptedDateRangeChange(dates);
    }
  };

  const onDateChange = (date: string, type: AcceptedRangeKind) => {
    switch (type) {
      case AcceptedRangeKind.From:
        setDateFrom(date);
        return;
      case AcceptedRangeKind.To:
        setDateTo(date);
        return;
    }
  };

  useEffect(() => {
    setDateFrom(props.acceptedFrom || INITIAL_DATE);
    setDateTo(props.acceptedTo || END_DATE);
  }, [props.acceptedFrom, props.acceptedTo]); /* eslint-disable-line react-hooks/exhaustive-deps */

  useEffect(() => {
    let times: AcceptedDate = {};
    if (dateTo !== END_DATE) {
      times[AcceptedRangeKind.To] = dateTo;
    }
    if (dateFrom !== INITIAL_DATE) {
      times[AcceptedRangeKind.From] = dateFrom;
    }
    updateAcceptedRange(times);
  }, [dateTo, dateFrom]); /* eslint-disable-line react-hooks/exhaustive-deps */

  return (
    <>
      <div className={`fw-bold text-uppercase text-primary ${styles.categoryTitle}`}>
        <small>Accepted</small>
      </div>

      <div className={`flex-column ${styles.mobileDateRange}`}>
        <div className="my-2">
          <label htmlFor="from" className={`form-label text-uppercase text-muted ${styles.label}`}>
            From:
          </label>
          <input
            type="date"
            className={`form-control form-control-sm rounded-0 ${styles.input}`}
            min={INITIAL_DATE}
            max={dateTo}
            id="from"
            value={dateFrom}
            onChange={(e: ChangeEvent<HTMLInputElement>) => onDateChange(e.target.value, AcceptedRangeKind.From)}
          />
        </div>
        <div>
          <label htmlFor="to" className={`form-label text-uppercase text-muted ${styles.label}`}>
            To:
          </label>
          <input
            type="date"
            className={`form-control form-control-sm rounded-0 ${styles.input}`}
            min={dateFrom}
            max={END_DATE}
            id="to"
            value={dateTo}
            onChange={(e: ChangeEvent<HTMLInputElement>) => onDateChange(e.target.value, AcceptedRangeKind.To)}
          />
        </div>
      </div>

      <div className={`mt-3 ${styles.desktopDateRange}`}>
        <AcceptedDateBtn
          date={dateFrom}
          min={INITIAL_DATE}
          max={dateTo}
          onDateChange={onDateChange}
          type={AcceptedRangeKind.From}
        />
        <div className="mt-3">
          <AcceptedDateBtn
            date={dateTo}
            min={dateFrom}
            max={END_DATE}
            onDateChange={onDateChange}
            type={AcceptedRangeKind.To}
          />
        </div>
      </div>
    </>
  );
};

export default AcceptedDateRange;
