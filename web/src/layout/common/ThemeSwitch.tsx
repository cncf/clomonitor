import { useContext } from 'react';
import { CgMoon, CgSun } from 'react-icons/cg';

import { AppContext, updateTheme } from '../../context/AppContextProvider';
import styles from './ThemeSwitch.module.css';

const ThemeSwitch = () => {
  const { ctx, dispatch } = useContext(AppContext);
  const { effective } = ctx.prefs.theme;
  const isLightActive = effective === 'light';

  const updateActiveTheme = () => {
    dispatch(updateTheme(isLightActive ? 'dark' : 'light'));
  };

  return (
    <>
      <div className="form-check form-switch ms-4 position-relative d-none d-sm-block cursorPointer">
        <div onClick={updateActiveTheme}>
          {isLightActive ? (
            <CgSun className={`text-light position-absolute ${styles.sunIcon}`} />
          ) : (
            <CgMoon className={`text-light position-absolute ${styles.moonIcon}`} />
          )}
        </div>

        <input
          className={`form-check-input cursorPointer ${styles.themeSwitch}`}
          type="checkbox"
          role="switch"
          id="theme"
          checked={effective === 'light'}
          onChange={updateActiveTheme}
        />
      </div>
    </>
  );
};

export default ThemeSwitch;
