import 'rc-slider/assets/index.css';

import { isArray, isUndefined, range } from 'lodash';
import moment from 'moment';
import Slider from 'rc-slider';
import { useContext, useEffect, useState } from 'react';

import { AppContext } from '../../../context/AppContextProvider';
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

interface Mark {
  [key: string | number]: string;
}

const AcceptedDateRange = (props: Props) => {
  const { ctx } = useContext(AppContext);
  const { effective } = ctx.prefs.theme;
  const [isLightActive, setIsLightActive] = useState<boolean>(effective === 'light');
  const [marks, setMarks] = useState<Mark>({});
  const [maxValue, setMaxValue] = useState<number>(1);
  const [years, setYears] = useState<string[]>([]);
  const [firstValue, setFirstValue] = useState<number>(0);
  const [secondValue, setSecondValue] = useState<number>(0);

  const calculateRange = () => {
    const yearsNumber = moment().diff(INITIAL_DATE, 'years');
    setMaxValue(yearsNumber);
    setSecondValue(yearsNumber);
    const visibleYears: Mark = {};
    const allYears: string[] = [];
    range(yearsNumber + 1).forEach((n: number) => {
      const year = moment(INITIAL_DATE).add(n, 'y');
      visibleYears[n.toString()] = year.format(n === 0 || n === yearsNumber ? 'YYYY' : "'YY");
      allYears.push(year.format('YYYY'));
    });
    setMarks(visibleYears);
    setYears(allYears);
  };

  const getDate = (index: number, fromDate: boolean): string | undefined => {
    let year: string | undefined;
    if ((fromDate && index !== 0) || (!fromDate && index !== maxValue)) {
      year = `${years[index]}-${fromDate ? '01-01' : '12-31'}`;
    }
    return year;
  };

  const getDateIndex = (fromDate: boolean, value?: string): number => {
    if (isUndefined(value)) return fromDate ? 0 : maxValue;
    const year = moment(value).format('YYYY');
    const yearIndex = years.findIndex((y: string) => y === year);
    if (yearIndex && yearIndex >= 0) {
      return yearIndex;
    } else {
      return fromDate ? 0 : maxValue;
    }
  };

  const handleChange = (value: number | number[]) => {
    if (isArray(value) && value.length === 2) {
      props.onAcceptedDateRangeChange({
        accepted_from: getDate(value[0], true),
        accepted_to: getDate(value[1], false),
      });
    }
  };

  useEffect(() => {
    setFirstValue(getDateIndex(true, props.acceptedFrom));
    setSecondValue(getDateIndex(false, props.acceptedTo));
  }, [props.acceptedFrom, props.acceptedTo]); /* eslint-disable-line react-hooks/exhaustive-deps */

  useEffect(() => {
    setIsLightActive(effective === 'light');
  }, [effective]);

  useEffect(() => {
    calculateRange();
  }, []);

  return (
    <>
      <div className={`fw-bold text-uppercase text-primary ${styles.categoryTitle}`}>
        <small>Accepted</small>
      </div>

      <div className={`mt-3 mb-5 ms-3 ms-md-2 me-4 me-md-2 me-xxl-5 ${styles.sliderWrapper}`}>
        <Slider
          range
          dots
          allowCross={false}
          step={1}
          min={0}
          max={maxValue}
          marks={marks}
          value={[firstValue, secondValue]}
          railStyle={{ backgroundColor: 'var(--color-black-10)' }}
          trackStyle={[{ backgroundColor: isLightActive ? '#695085' : '#cbd3da' }]}
          handleStyle={{
            borderColor: isLightActive ? '#2a0552' : '#a3a3a6',
            backgroundColor: isLightActive ? 'var(--bs-white)' : '#f9f9f9',
          }}
          activeDotStyle={{ borderColor: isLightActive ? '#2a0552' : '#a3a3a6' }}
          onAfterChange={handleChange}
          onChange={(value: number | number[]) => {
            if (isArray(value) && value.length === 2) {
              setFirstValue(value[0]);
              setSecondValue(value[1]);
            }
          }}
        />
      </div>
    </>
  );
};

export default AcceptedDateRange;
