import { ChangeEvent, useContext, useRef } from 'react';

import { AppContext } from '../../context/AppContextProvider';
import styles from './PaginationLimit.module.css';

const AVAILABLE_LIMITS = [20, 40, 60];

interface Props {
  onPaginationLimitChange: (limit: number) => void;
}

const PaginationLimit = (props: Props) => {
  const { ctx } = useContext(AppContext);
  const { limit } = ctx.prefs.search;
  const selectEl = useRef<HTMLSelectElement>(null);

  const handleChange = (event: ChangeEvent<HTMLSelectElement>) => {
    props.onPaginationLimitChange(parseInt(event.target.value));
    forceBlur();
  };

  const forceBlur = (): void => {
    if (selectEl && selectEl.current) {
      selectEl.current.blur();
    }
  };

  return (
    <div className="d-none d-md-flex flex-nowrap align-items-center lh-1">
      <label className="form-label me-2 mb-0">Show:</label>
      <select
        ref={selectEl}
        className={`form-select form-select-sm rounded-0 ${styles.select}`}
        value={limit}
        onChange={handleChange}
      >
        {AVAILABLE_LIMITS.map((opt: number) => {
          return (
            <option value={opt} key={`limit_${opt}`}>
              {opt}
            </option>
          );
        })}
      </select>
    </div>
  );
};

export default PaginationLimit;
