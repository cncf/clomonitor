import { DateRangeFilter, DateRangeOpts, Foundation } from 'clo-ui';
import { isEmpty, isUndefined } from 'lodash';
import React, { useEffect, useState } from 'react';
import { IoMdCloseCircleOutline } from 'react-icons/io';

import { FILTERS, MATURITY_FILTERS } from '../../../data';
import { FilterKind, FiltersSection, ReportOption } from '../../../types';
import Checks from './checks';
import Section from './Section';
interface FiltersProp {
  [key: string]: string[];
}

interface Props {
  visibleTitle: boolean;
  acceptedFrom?: string;
  acceptedTo?: string;
  activeFilters: {
    [key: string]: string[];
  };
  onChange: (name: string, value: string, checked: boolean) => void;
  onChecksChange: (filters: FiltersProp) => void;
  onAcceptedDateRangeChange: (dates: DateRangeOpts) => void;
  onResetFilters?: () => void;
  device: string;
}

const Filters = (props: Props) => {
  const [selectedFoundation, setSelectedFoundation] = useState<Foundation | null>(null);

  useEffect(() => {
    if (
      !isUndefined(props.activeFilters[FilterKind.Foundation]) &&
      props.activeFilters[FilterKind.Foundation].length === 1
    ) {
      setSelectedFoundation(props.activeFilters[FilterKind.Foundation][0] as Foundation);
    } else {
      setSelectedFoundation(null);
    }
  }, [props.activeFilters]);

  return (
    <>
      {props.visibleTitle && (
        <div className="d-flex flex-row align-items-center justify-content-between pb-2 mb-4 border-bottom border-1">
          <div className="h6 text-uppercase mb-0 lh-base text-primary fw-bold">Filters</div>
          {(!isEmpty(props.activeFilters) || !isUndefined(props.acceptedFrom) || !isUndefined(props.acceptedTo)) && (
            <button className="btn btn-link text-primary" onClick={props.onResetFilters} aria-label="Reset filters">
              <div className="d-flex flex-row align-items-center">
                <IoMdCloseCircleOutline className="me-2" />

                <small>Reset</small>
              </div>
            </button>
          )}
        </div>
      )}

      {FILTERS.map((section: FiltersSection) => {
        return (
          <React.Fragment key={`sec_${section.name}`}>
            <Section
              device={props.device}
              activeFilters={props.activeFilters[section.name]}
              section={section}
              onChange={props.onChange}
            />
            {section.name === FilterKind.Foundation &&
              selectedFoundation &&
              !isUndefined(MATURITY_FILTERS[selectedFoundation]) && (
                <Section
                  device={props.device}
                  activeFilters={props.activeFilters[FilterKind.Maturity]}
                  section={MATURITY_FILTERS[selectedFoundation]!}
                  onChange={props.onChange}
                />
              )}
          </React.Fragment>
        );
      })}

      <div>
        <Checks
          activePassingChecks={props.activeFilters[FilterKind.PassingCheck] as ReportOption[]}
          activeNotPassingChecks={props.activeFilters[FilterKind.NotPassingCheck] as ReportOption[]}
          onChecksChange={props.onChecksChange}
          onChange={props.onChange}
        />
      </div>

      <DateRangeFilter
        initialDate="2016-01-01"
        from={props.acceptedFrom}
        to={props.acceptedTo}
        onDateRangeChange={props.onAcceptedDateRangeChange}
      />
    </>
  );
};

export default Filters;
