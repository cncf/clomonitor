import { isEmpty, isUndefined } from 'lodash';
import moment from 'moment';
import { Fragment } from 'react';
import { IoMdCloseCircleOutline } from 'react-icons/io';

import { FILTER_CATEGORY_NAMES, FILTERS, FOUNDATIONS, REPORT_OPTIONS } from '../../data';
import { Filter, FilterKind, FiltersSection, Foundation, ReportOption } from '../../types';
import styles from './SelectedFilters.module.css';

interface Props {
  acceptedFrom?: string;
  acceptedTo?: string;
  filters: { [key: string]: string[] };
  onChange: (name: string, value: string, checked: boolean) => void;
  onAcceptedDateRangeChange: (dates: any) => void;
}

const SelectedFilters = (props: Props) => {
  if (isEmpty(props.filters) && isUndefined(props.acceptedFrom) && isUndefined(props.acceptedTo)) return null;

  const getFilterName = (type: FilterKind, filter: string): string => {
    switch (type) {
      case FilterKind.Foundation:
        return FOUNDATIONS[filter as Foundation].name;

      case FilterKind.Maturity:
        return filter;

      case FilterKind.PassingCheck:
      case FilterKind.NotPassingCheck:
        return REPORT_OPTIONS[filter as ReportOption].shortName || REPORT_OPTIONS[filter as ReportOption].name;

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

  const getFormatDate = (date?: string): string => {
    if (isUndefined(date)) return '';
    return moment(date).format('MMM D, YYYY');
  };

  const compoundAcceptedFilter = (): string => {
    if (props.acceptedFrom && props.acceptedTo) {
      return `${getFormatDate(props.acceptedFrom)} - ${getFormatDate(props.acceptedTo)}`;
    } else {
      return `${isUndefined(props.acceptedFrom) ? '≤' : '≥'} ${getFormatDate(props.acceptedFrom || props.acceptedTo)}`;
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
            const categoryName = FILTER_CATEGORY_NAMES[category as FilterKind];
            return (
              <Fragment key={`filter_${category}`}>
                {props.filters[category].map((filter: string) => {
                  const filterName = getFilterName(category as FilterKind, filter);
                  return (
                    <span
                      role="listitem"
                      className={`badge bg-secondary rounded-0 text-light me-3 my-1 ${styles.badge} lightBorder`}
                      key={`filter_${category}_${filter}`}
                    >
                      <div className="d-flex flex-row align-items-baseline">
                        <div className={styles.content}>
                          <small className="text-uppercase fw-normal me-2">{categoryName}:</small>
                          <span className="text-capitalize">{filterName}</span>
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
