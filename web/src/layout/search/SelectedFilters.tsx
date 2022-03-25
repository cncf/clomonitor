import { isEmpty, isUndefined } from 'lodash';
import moment from 'moment';
import { Fragment } from 'react';
import { IoMdCloseCircleOutline } from 'react-icons/io';

import { FILTERS } from '../../data';
import { Category, Filter, FilterKind, FiltersSection, Maturity } from '../../types';
import styles from './SelectedFilters.module.css';

interface Props {
  acceptedFrom?: string;
  acceptedTo?: string;
  filters: { [key: string]: (string | number)[] };
  onChange: (name: string, value: string, checked: boolean) => void;
  onAcceptedDateRangeChange: (dates: any) => void;
}

const SelectedFilters = (props: Props) => {
  if (isEmpty(props.filters) && isUndefined(props.acceptedFrom) && isUndefined(props.acceptedTo)) return null;

  const getFilterName = (type: FilterKind, filter: string | number): string => {
    switch (type) {
      case FilterKind.Category:
        return Category[parseInt(filter as string)];
      case FilterKind.Maturity:
        return Maturity[parseInt(filter as string)];
      case FilterKind.Rating:
        let ratingName: string = '';
        const ratings = FILTERS.find((sec: FiltersSection) => sec.name === type);
        if (ratings) {
          const rating = ratings.filters.find((f: Filter) => f.name === filter);
          if (rating) {
            ratingName = `${rating.label} ${rating.legend}`;
          }
        }
        return ratingName;
    }
  };

  const getYearFromDate = (date?: string): string => {
    if (isUndefined(date)) return '';
    return moment(date).format('YYYY');
  };

  const compoundAcceptedFilter = (): string => {
    if (props.acceptedFrom && props.acceptedTo) {
      const formattedAcceptedFrom = getYearFromDate(props.acceptedFrom);
      const formattedAcceptedTo = getYearFromDate(props.acceptedTo);
      if (formattedAcceptedFrom === formattedAcceptedTo) {
        return formattedAcceptedFrom;
      } else {
        return `${formattedAcceptedFrom} - ${formattedAcceptedTo}`;
      }
    } else {
      return `${isUndefined(props.acceptedFrom) ? '≤' : '≥'} ${getYearFromDate(
        props.acceptedFrom || props.acceptedTo
      )}`;
    }
  };

  return (
    <div className="d-none d-md-block mt-2">
      <div className="d-flex flex-row justify-content-start align-items-baseline">
        <div className="me-3">Filters:</div>
        <div role="list" className={`position-relative ${styles.badges}`}>
          {(!isUndefined(props.acceptedFrom) || !isUndefined(props.acceptedTo)) && (
            <span
              role="listitem"
              className={`badge bg-secondary rounded-0 text-light me-3 my-1 ${styles.badge} lightBorder`}
            >
              <div className="d-flex flex-row align-items-baseline">
                <div className={styles.content}>
                  <small className="text-uppercase fw-normal me-2">Accepted:</small>
                  {compoundAcceptedFilter()}
                </div>
                <button
                  className={`btn btn-link btn-sm lh-1 ${styles.btn}`}
                  onClick={() => props.onAcceptedDateRangeChange({ accepted_from: undefined, accepted_to: undefined })}
                  aria-label="Remove accepted filter"
                >
                  <IoMdCloseCircleOutline />
                </button>
              </div>
            </span>
          )}
          {Object.keys(props.filters).map((category: string) => {
            return (
              <Fragment key={`filter_${category}`}>
                {props.filters[category].map((filter: string | number) => {
                  const filterName = getFilterName(category as FilterKind, filter);
                  return (
                    <span
                      role="listitem"
                      className={`badge bg-secondary rounded-0 text-light me-3 my-1 ${styles.badge} lightBorder`}
                      key={`filter_${category}_${filter}`}
                    >
                      <div className="d-flex flex-row align-items-baseline">
                        <div className={styles.content}>
                          <small className="text-uppercase fw-normal me-2">{category}:</small>
                          {filterName}
                        </div>
                        <button
                          className={`btn btn-link btn-sm lh-1 ${styles.btn}`}
                          onClick={() => props.onChange(category, filter as string, false)}
                          aria-label={`Remove ${filterName} filter`}
                        >
                          <IoMdCloseCircleOutline />
                        </button>
                      </div>
                    </span>
                  );
                })}
              </Fragment>
            );
          })}
        </div>
      </div>
    </div>
  );
};

export default SelectedFilters;
