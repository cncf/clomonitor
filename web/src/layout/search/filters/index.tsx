import { IoMdCloseCircleOutline } from 'react-icons/io';

import { FILTERS } from '../../../data';
import { FiltersSection } from '../../../types';
import Section from './Section';

interface Props {
  visibleTitle: boolean;
  activeFilters: {
    [key: string]: (string | number)[];
  };
  onChange: (name: string, value: string, checked: boolean) => void;
  onResetFilters?: () => void;
  device: string;
}

const Filters = (props: Props) => {
  return (
    <>
      {props.visibleTitle && (
        <div className="d-flex flex-row align-items-center justify-content-between pb-2 mb-4 border-bottom">
          <div className="h6 text-uppercase mb-0 lh-base text-primary fw-bold">Filters</div>
          <button className="btn btn-link text-primary" onClick={props.onResetFilters} aria-label="Reset filters">
            <div className="d-flex flex-row align-items-center">
              <IoMdCloseCircleOutline className="me-2" />

              <small>Reset</small>
            </div>
          </button>
        </div>
      )}

      {FILTERS.map((section: FiltersSection) => (
        <Section
          device={props.device}
          activeFilters={props.activeFilters[section.name]}
          section={section}
          key={`sec_${section.name}`}
          onChange={props.onChange}
        />
      ))}
    </>
  );
};

export default Filters;
