import { isUndefined } from 'lodash';
import { ChangeEvent } from 'react';

import { Filter, FiltersSection } from '../../../types';
import CheckBox from '../../common/Checkbox';
import styles from './Section.module.css';

interface Props {
  section: FiltersSection;
  device: string;
  activeFilters?: string[];
  onChange: (name: string, value: string, checked: boolean) => void;
}

const Section = (props: Props) => {
  return (
    <>
      <div className={`fw-bold text-uppercase text-primary ${styles.categoryTitle}`}>
        <small>{props.section.title}</small>
      </div>

      <div className="mt-2">
        {props.section.filters.map((filter: Filter) => {
          return (
            <CheckBox
              key={`filter_${filter.name.toString()}`}
              name={props.section.name}
              value={filter.name.toString()}
              labelClassName="mw-100"
              legend={filter.legend}
              label={filter.label}
              icon={<span className={`position-relative ${styles.decorator}`}>{filter.decorator}</span>}
              device={props.device}
              checked={!isUndefined(props.activeFilters) && props.activeFilters.includes(filter.name.toString())}
              onChange={(e: ChangeEvent<HTMLInputElement>) =>
                props.onChange(e.target.name, e.target.value, e.target.checked)
              }
            />
          );
        })}
      </div>
    </>
  );
};

export default Section;
