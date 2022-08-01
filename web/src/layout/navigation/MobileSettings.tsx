import classNames from 'classnames';
import { useRef, useState } from 'react';
import { FaChartPie, FaFileAlt } from 'react-icons/fa';
import { HiDotsVertical } from 'react-icons/hi';
import { Link } from 'react-router-dom';

import useOutsideClick from '../../hooks/useOutsideClick';
import ExternalLink from '../common/ExternalLink';
import styles from './MobileSettings.module.css';
import ThemeMode from './ThemeMode';

const MobileSettings = () => {
  const [visibleDropdown, setVisibleDropdown] = useState(false);
  const ref = useRef(null);
  useOutsideClick([ref], visibleDropdown, () => setVisibleDropdown(false));

  const closeDropdown = () => {
    setVisibleDropdown(false);
  };

  return (
    <div ref={ref} className="d-flex d-md-none ms-auto position-relative">
      <button
        className={`btn btn-sm btn-link text-white rounded-0 lh-1 ms-3 ${styles.btn}`}
        type="button"
        onClick={() => setVisibleDropdown(!visibleDropdown)}
        aria-label="Mobile settings button"
        aria-expanded={visibleDropdown}
      >
        <HiDotsVertical />
      </button>

      <div role="menu" className={classNames('dropdown-menu rounded-0', styles.dropdown, { show: visibleDropdown })}>
        <ThemeMode onChange={closeDropdown} device="mobile" />

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
