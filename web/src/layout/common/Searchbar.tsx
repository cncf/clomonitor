import { isNull } from 'lodash';
import { ChangeEvent, KeyboardEvent, useRef, useState } from 'react';
import { FiSearch } from 'react-icons/fi';
import { useNavigate } from 'react-router-dom';

import { prepareQueryString } from '../../utils/prepareQueryString';
import styles from './Searchbar.module.css';

interface Props {
  classNameWrapper?: string;
}

const Searchbar = (props: Props) => {
  const navigate = useNavigate();
  const inputEl = useRef<HTMLInputElement>(null);
  const [value, setValue] = useState<string>('');

  const onKeyDown = (e: KeyboardEvent<HTMLInputElement>): void => {
    if (e.key === 'Enter') {
      navigate({
        pathname: '/search',
        search: prepareQueryString({
          pageNumber: 1,
          text: value,
          filters: {},
        }),
      });
      setValue('');
      forceBlur();
    }
  };

  const forceBlur = (): void => {
    if (!isNull(inputEl) && !isNull(inputEl.current)) {
      inputEl.current.blur();
    }
  };

  return (
    <div className={`position-relative ${props.classNameWrapper}`}>
      <div
        className={`d-flex align-items-center overflow-hidden searchBar lh-base bg-white mx-auto ${styles.searchBar} search`}
      >
        <div className={`d-flex align-items-center ${styles.iconWrapper} lightIcon`}>
          <FiSearch />
        </div>

        <input
          ref={inputEl}
          className={`flex-grow-1 ps-2 ps-md-0 border-0 shadow-none bg-transparent lh-base ${styles.input}`}
          type="text"
          value={value}
          autoComplete="off"
          autoCorrect="off"
          autoCapitalize="none"
          spellCheck="false"
          placeholder="Search projects"
          onKeyDown={onKeyDown}
          onChange={(e: ChangeEvent<HTMLInputElement>) => setValue(e.target.value)}
        />
      </div>
    </div>
  );
};

export default Searchbar;
