import classNames from 'classnames';
import { useContext, useRef, useState } from 'react';
import { FaChartPie, FaFileAlt } from 'react-icons/fa';
import { FiMoon, FiSun } from 'react-icons/fi';
import { HiDotsVertical } from 'react-icons/hi';
import { Link } from 'react-router-dom';

import { AppContext, updateTheme } from '../../context/AppContextProvider';
import useOutsideClick from '../../hooks/useOutsideClick';
import ExternalLink from '../common/ExternalLink';
import styles from './MobileSettings.module.css';

const MobileSettings = () => {
  const [visibleDropdown, setVisibleDropdown] = useState(false);
  const ref = useRef(null);
  const { ctx, dispatch } = useContext(AppContext);
  const { effective } = ctx.prefs.theme;
  useOutsideClick([ref], visibleDropdown, () => setVisibleDropdown(false));

  const onHandleChange = (value: string) => {
    dispatch(updateTheme(value));
    closeDropdown();
  };

  const closeDropdown = () => {
    setVisibleDropdown(false);
  };

  return (
    <div ref={ref} className="d-flex d-md-none ms-auto position-relative">
      <button
        className={`btn btn-sm btn-primary text-white rounded-0 lh-1 ms-3 ${styles.btn}`}
        type="button"
        onClick={() => setVisibleDropdown(!visibleDropdown)}
        aria-label="Mobile settings button"
        aria-expanded={visibleDropdown}
      >
        <HiDotsVertical />
      </button>

      <div role="menu" className={classNames('dropdown-menu rounded-0', styles.dropdown, { show: visibleDropdown })}>
        <div className="px-3 py-2 lightText text-secondary text-uppercase fw-bold">Theme</div>
        <div className="dropdown-item">
          <div className="form-check">
            <input
              id="theme-light"
              name="light"
              className={`form-check-input ${styles.input}`}
              type="radio"
              value="light"
              onChange={() => onHandleChange('light')}
              aria-checked={effective === 'light'}
              tabIndex={-1}
              checked={effective === 'light'}
            />
            <label className={`form-check-label fw-bold w-100 ${styles.label}`} htmlFor="theme-light">
              <FiSun className={`mx-1 position-relative ${styles.icon}`} />
              Light
            </label>
          </div>
        </div>

        <div className="dropdown-item">
          <div className="form-check">
            <input
              id="theme-dark"
              name="dark"
              className={`form-check-input ${styles.input}`}
              type="radio"
              value="dark"
              onChange={() => onHandleChange('dark')}
              aria-checked={effective === 'dark'}
              tabIndex={-1}
              checked={effective === 'dark'}
            />
            <label className={`form-check-label fw-bold w-100 ${styles.label}`} htmlFor="theme-dark">
              <FiMoon className={`mx-1 position-relative ${styles.icon}`} />
              Dark
            </label>
          </div>
        </div>

        <hr />

        <div className="dropdown-item mb-2">
          <ExternalLink
            className="text-decoration-none fw-bold d-inline-block w-100"
            href="/docs"
            label="Open documentation"
            target="_self"
          >
            <div className="d-flex flex-row align-items-center py-1">
              <FaFileAlt />
              <div className="ms-2">Documentation</div>
            </div>
          </ExternalLink>
        </div>

        <div className="dropdown-item mb-2">
          <Link className="text-decoration-none fw-bold d-inline-block w-100" to="/stats" onClick={closeDropdown}>
            <div className="d-flex flex-row align-items-center py-1">
              <FaChartPie />
              <div className="ms-2">Statistics</div>
            </div>
          </Link>
        </div>
      </div>
    </div>
  );
};

export default MobileSettings;
