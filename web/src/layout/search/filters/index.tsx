import { isEmpty, isUndefined } from 'lodash';
import React from 'react';
import { IoMdCloseCircleOutline } from 'react-icons/io';

import { FILTERS } from '../../../data';
import { FilterKind, FiltersSection, ReportOption } from '../../../types';
import AcceptedDateRange from './AcceptedDateRange';
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
  onAcceptedDateRangeChange: (dates: any) => void;
  onResetFilters?: () => void;
  device: string;
}

const Filters = (props: Props) => {
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

      {FILTERS.map((section: FiltersSection) => (
        <React.Fragment key={`sec_${section.name}`}>
          <Section
            device={props.device}
            activeFilters={props.activeFilters[section.name]}
            section={section}
            onChange={props.onChange}
          />
        </React.Fragment>
      ))}

      <div>
        <Checks
          activePassingChecks={props.activeFilters[FilterKind.PassingCheck] as ReportOption[]}
          activeNotPassingChecks={props.activeFilters[FilterKind.NotPassingCheck] as ReportOption[]}
          onChecksChange={props.onChecksChange}
          onChange={props.onChange}
        />
      </div>

      <AcceptedDateRange
        acceptedFrom={props.acceptedFrom}
        acceptedTo={props.acceptedTo}
        onAcceptedDateRangeChange={props.onAcceptedDateRangeChange}
      />
    </>
  );
};

export default Filters;
