import { isNull } from 'lodash';
import { ChangeEvent, Dispatch, KeyboardEvent, SetStateAction, useEffect, useRef, useState } from 'react';
import { FiSearch } from 'react-icons/fi';
import { IoCloseSharp } from 'react-icons/io5';
import { useNavigate, useSearchParams } from 'react-router-dom';

import { prepareQueryString } from '../../utils/prepareQueryString';
import styles from './Searchbar.module.css';

interface Props {
  setScrollPosition: Dispatch<SetStateAction<number | undefined>>;
  classNameWrapper?: string;
}

const Searchbar = (props: Props) => {
  const navigate = useNavigate();
  const [searchParams] = useSearchParams();
  const inputEl = useRef<HTMLInputElement>(null);
  const [value, setValue] = useState<string>('');
  const [currentSearch, setCurrentSearch] = useState<string | null>(null);

  const onKeyDown = (e: KeyboardEvent<HTMLInputElement>): void => {
    if (e.key === 'Enter') {
      search();
    }
  };

  const forceBlur = (): void => {
    if (!isNull(inputEl) && !isNull(inputEl.current)) {
      inputEl.current.blur();
    }
  };

  const search = () => {
    props.setScrollPosition(0);
    navigate({
      pathname: '/search',
      search: prepareQueryString({
        pageNumber: 1,
        text: value,
        filters: {},
      }),
    });
    forceBlur();
  };

  const cleanSearch = () => {
    if (currentSearch === value) {
      props.setScrollPosition(0);
      navigate({
        pathname: '/search',
        search: prepareQueryString({
          pageNumber: 1,
          text: '',
          filters: {},
        }),
      });
      forceBlur();
    } else {
      setValue('');
    }
  };

  useEffect(() => {
    const text = searchParams.get('text');
    setValue(text || '');
    setCurrentSearch(text);
  }, [searchParams]);

  return (
    <div className={`position-relative ${props.classNameWrapper}`}>
      <div
        className={`d-flex align-items-center overflow-hidden searchBar lh-base bg-white mx-auto ${styles.searchBar} search`}
      >
        <input
          ref={inputEl}
          className={`flex-grow-1 ps-2 ps-md-3 border-0 shadow-none bg-transparent lh-base ${styles.input}`}
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

        {value !== '' && (
          <>
            <button className={`btn btn-link lh-1 px-2 ${styles.btnIcon}`} onClick={cleanSearch}>
              <div className="text-muted lightIcon">
                <IoCloseSharp />
              </div>
            </button>
            <div className={`vr ${styles.vr}`} />
          </>
        )}

        <button className={`btn btn-link lh-1 px-2 ${styles.btnIcon}`} onClick={search}>
          <div className={`${styles.iconWrapper} lightIcon`}>
            <FiSearch />
          </div>
        </button>
      </div>
    </div>
  );
};

export default Searchbar;
