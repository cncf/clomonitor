import { isNull } from 'lodash';
import { ChangeEvent, useRef } from 'react';

import { SortBy, SortDirection } from '../../types';
import styles from './SortOptions.module.css';

interface Props {
  by: SortBy;
  direction: SortDirection;
  onSortChange: (by: SortBy, direction: SortDirection) => void;
}

interface Option {
  label: string;
  by: SortBy;
  direction: SortDirection;
}

const SORT_OPTS: Option[] = [
  {
    label: 'Alphabetically (A-Z)',
    by: SortBy.Name,
    direction: SortDirection.ASC,
  },
  {
    label: 'Alphabetically (Z-A)',
    by: SortBy.Name,
    direction: SortDirection.DESC,
  },
  {
    label: 'Score (highest first)',
    by: SortBy.Score,
    direction: SortDirection.DESC,
  },
  {
    label: 'Score (lowest first)',
    by: SortBy.Score,
    direction: SortDirection.ASC,
  },
];

const SortOptions = (props: Props) => {
  const selectEl = useRef<HTMLSelectElement>(null);

  const handleChange = (event: ChangeEvent<HTMLSelectElement>) => {
    const value = event.target.value.split('_');
    props.onSortChange(value[0] as SortBy, value[1] as SortDirection);
    forceBlur();
  };

  const forceBlur = (): void => {
    if (!isNull(selectEl) && !isNull(selectEl.current)) {
      selectEl.current.blur();
    }
  };

  return (
    <div className="d-flex flex-nowrap align-items-center me-2 me-md-4">
      <label className="form-label me-2 mb-0">Sort:</label>
      <select
        ref={selectEl}
        className={`form-select form-select-sm rounded-0 ${styles.select}`}
        value={`${props.by}_${props.direction}`}
        onChange={handleChange}
      >
        {SORT_OPTS.map((opt: Option) => (
          <option key={`sort_${opt.label}`} value={`${opt.by}_${opt.direction}`}>
            {opt.label}
          </option>
        ))}
        ;
      </select>
    </div>
  );
};

export default SortOptions;
