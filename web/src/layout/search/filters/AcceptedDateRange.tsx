import 'react-date-range/dist/styles.css'; // main style file
import 'react-date-range/dist/theme/default.css'; // theme css file

import classNames from 'classnames';
import { isEmpty } from 'lodash';
import moment from 'moment';
import { ChangeEvent, useEffect, useRef, useState } from 'react';
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
  const startDate = moment('2016-01-01').toDate();
  const endDate = moment().toDate();
  const END_DATE = moment(endDate).format('YYYY-MM-DD');
  const [dateFrom, setDateFrom] = useState<string>(INITIAL_DATE);
  const [dateTo, setDateTo] = useState(END_DATE);
  const ref = useRef<HTMLDivElement>(null);
  const [showCalendar, setShowCalendar] = useState<boolean>(false);
  useOutsideClick([ref], showCalendar, () => setShowCalendar(false));

  const [ranges, setRanges] = useState<Range[]>([
    {
      startDate: startDate,
      endDate: endDate,
      key: 'selection',
    },
  ]);

  const handleChange = (data: RangeKeyDict) => {
    if (data.selection) {
      setRanges([data.selection]);

      let times: AcceptedDate = {};

      if (!moment(endDate).isSame(moment(data.selection.endDate))) {
        times['accepted_to'] = moment(data.selection.endDate!).format('YYYY-MM-DD');
      }

      if (!moment(startDate).isSame(moment(data.selection.startDate))) {
        times['accepted_from'] = moment(data.selection.startDate!).format('YYYY-MM-DD');
      }

      updateAcceptedRange(times);
    }
  };

  const updateAcceptedRange = (dates: AcceptedDate) => {
    if (!isEmpty(dates)) {
      props.onAcceptedDateRangeChange(dates);
    }
  };

  const onChangeDate = (e: ChangeEvent<HTMLInputElement>, type: string) => {
    switch (type) {
      case 'from':
        setDateFrom(e.target.value);
        return;
      case 'to':
        setDateTo(e.target.value);
        return;
    }
  };

  useEffect(() => {
    setRanges([
      {
        startDate: moment(props.acceptedFrom || INITIAL_DATE).toDate(),
        endDate: moment(props.acceptedTo || moment()).toDate(),
        key: 'selection',
      },
    ]);
    setDateFrom(props.acceptedFrom || INITIAL_DATE);
    setDateTo(props.acceptedTo || END_DATE);
  }, [props.acceptedFrom, props.acceptedTo]); /* eslint-disable-line react-hooks/exhaustive-deps */

  useEffect(() => {
    let times: AcceptedDate = {};
    if (dateTo !== END_DATE) {
      times['accepted_to'] = dateTo;
    }
    if (dateFrom !== INITIAL_DATE) {
      times['accepted_from'] = dateFrom;
    }
    updateAcceptedRange(times);
  }, [dateTo, dateFrom]); /* eslint-disable-line react-hooks/exhaustive-deps */

  return (
    <>
      <div className={`fw-bold text-uppercase text-primary ${styles.categoryTitle}`}>
        <small>Accepted</small>
      </div>

      <div className="d-flex flex-column d-md-none">
        <div className="my-2">
          <label htmlFor="from" className={`form-label text-uppercase text-muted ${styles.label}`}>
            From:
          </label>
          <input
            type="date"
            className="form-control form-control-sm rounded-0"
            min={INITIAL_DATE}
            max={dateTo}
            id="from"
            value={dateFrom}
            onChange={(e: ChangeEvent<HTMLInputElement>) => onChangeDate(e, 'from')}
          />
        </div>
        <div>
          <label htmlFor="to" className={`form-label text-uppercase text-muted ${styles.label}`}>
            To:
          </label>
          <input
            type="date"
            className="form-control form-control-sm rounded-0"
            min={dateFrom}
            max={END_DATE}
            id="to"
            value={dateTo}
            onChange={(date: any) => onChangeDate(date, 'to')}
          />
        </div>
      </div>

      <div className="d-none d-md-block mt-3">
        <button
          className={`btn btn-sm btn-outline-secondary rounded-0 w-100 ${styles.dateBtn}`}
          onClick={() => setShowCalendar(!showCalendar)}
          aria-label="Open calendar"
        >
          <span className="d-none d-xxl-block">
            {moment(ranges[0].startDate!).format('MMM D, YYYY')} - {moment(ranges[0].endDate!).format('MMM D, YYYY')}
          </span>
          <span className="d-block d-xxl-none">
            {moment(ranges[0].startDate!).format('MMM D, YY')} - {moment(ranges[0].endDate!).format('MMM D, YY')}
          </span>
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
            minDate={startDate}
            maxDate={endDate}
          />
        </div>
      </div>
    </>
  );
};

export default AcceptedDateRange;
