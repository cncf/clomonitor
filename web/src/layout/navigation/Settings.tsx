import classNames from 'classnames';
import { useOutsideClick } from 'clo-ui/hooks/useOutsideClick';
import { useRef, useState } from 'react';
import { FaCog } from 'react-icons/fa';

import styles from './Settings.module.css';
import ThemeMode from './ThemeMode';

const Settings = () => {
  const [visibleDropdown, setVisibleDropdown] = useState(false);
  const ref = useRef(null);
  useOutsideClick([ref], visibleDropdown, () => setVisibleDropdown(false));

  return (
    <div ref={ref} className="ms-2 position-relative">
      <button
        className={`btn btn-sm btn-link text-white rounded-0 lh-1 ms-3 ${styles.btn}`}
        type="button"
        onClick={() => setVisibleDropdown(!visibleDropdown)}
        aria-label="Settings button"
        aria-expanded={visibleDropdown}
      >
        <FaCog />
      </button>

      <div role="menu" className={classNames('dropdown-menu rounded-0', styles.dropdown, { show: visibleDropdown })}>
        <div className={`dropdown-arrow ${styles.arrow}`} />
        <ThemeMode closeDropdown={() => setVisibleDropdown(false)} device="desktop" />
      </div>
    </div>
  );
};

export default Settings;
