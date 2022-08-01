import { isUndefined } from 'lodash';
import { useContext } from 'react';
import { FiMoon, FiSun } from 'react-icons/fi';
import { GoBrowser } from 'react-icons/go';

import { AppContext, updateTheme } from '../../context/AppContextProvider';
import styles from './ThemeMode.module.css';

interface Props {
  onChange?: () => void;
  device: string;
}

const ThemeMode = (props: Props) => {
  const { ctx, dispatch } = useContext(AppContext);
  const { configured } = ctx.prefs.theme;

  const onHandleChange = (value: string) => {
    dispatch(updateTheme(value));
    if (!isUndefined(props.onChange)) {
      props.onChange();
    }
  };

  return (
    <>
      <div className="px-3 py-2 lightText text-secondary text-uppercase fw-bold">Theme</div>

      <div className="dropdown-item">
        <div className="form-check">
          <input
            id={`theme-${props.device}-automatic`}
            name="light"
            className={`form-check-input ${styles.input}`}
            type="radio"
            value="light"
            onChange={() => onHandleChange('automatic')}
            aria-checked={configured === 'automatic'}
            tabIndex={-1}
            checked={configured === 'automatic'}
          />
          <label className="form-check-label w-100" htmlFor={`theme-${props.device}-automatic`}>
            <GoBrowser className="mx-1 position-relative" />
            Automatic
          </label>
        </div>
      </div>

      <div className="dropdown-item">
        <div className="form-check">
          <input
            id={`theme-${props.device}-light`}
            name="light"
            className={`form-check-input ${styles.input}`}
            type="radio"
            value="light"
            onChange={() => onHandleChange('light')}
            aria-checked={configured === 'light'}
            tabIndex={-1}
            checked={configured === 'light'}
          />
          <label className="form-check-label w-100" htmlFor={`theme-${props.device}-light`}>
            <FiSun className="mx-1 position-relative" />
            Light
          </label>
        </div>
      </div>

      <div className="dropdown-item">
        <div className="form-check">
          <input
            id={`theme-${props.device}-dark`}
            name="dark"
            className={`form-check-input ${styles.input}`}
            type="radio"
            value="dark"
            onChange={() => onHandleChange('dark')}
            aria-checked={configured === 'dark'}
            tabIndex={-1}
            checked={configured === 'dark'}
          />
          <label className="form-check-label w-100" htmlFor={`theme-${props.device}-dark`}>
            <FiMoon className="mx-1 position-relative" />
            Dark
          </label>
        </div>
      </div>
    </>
  );
};

export default ThemeMode;
