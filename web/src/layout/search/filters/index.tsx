import { isEmpty, isUndefined } from 'lodash';
import React from 'react';
import { IoMdCloseCircleOutline } from 'react-icons/io';

import { FILTERS } from '../../../data';
import { FiltersSection } from '../../../types';
import AcceptedDateRange from './AcceptedDateRange';
import Section from './Section';

interface Props {
  visibleTitle: boolean;
  acceptedFrom?: string;
  acceptedTo?: string;
  activeFilters: {
    [key: string]: (string | number)[];
  };
  onChange: (name: string, value: string, checked: boolean) => void;
  onAcceptedDateRangeChange: (dates: any) => void;
  onResetFilters?: () => void;
  device: string;
}

const Filters = (props: Props) => {
  return (
    <>
      {props.visibleTitle && (
        <div className="d-flex flex-row align-items-center justify-content-between pb-2 mb-4 border-bottom">
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

      {FILTERS.map((section: FiltersSection, index: number) => (
        <React.Fragment key={`sec_${section.name}`}>
          <Section
            device={props.device}
            activeFilters={props.activeFilters[section.name]}
            section={section}
            onChange={props.onChange}
          />
          {index === 0 && (
            <AcceptedDateRange
              acceptedFrom={props.acceptedFrom}
              acceptedTo={props.acceptedTo}
              onAcceptedDateRangeChange={props.onAcceptedDateRangeChange}
            />
          )}
        </React.Fragment>
      ))}
    </>
  );
};

export default Filters;
