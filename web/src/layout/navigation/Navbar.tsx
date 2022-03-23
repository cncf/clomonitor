import { Dispatch, SetStateAction } from 'react';
import { Link } from 'react-router-dom';

import logo from '../../media/clomonitor.svg';
import ThemeSwitch from '../common/ThemeSwitch';
import Searchbar from '../navigation/Searchbar';
import MobileSettings from './MobileSettings';
import styles from './Navbar.module.css';

interface Props {
  setScrollPosition: Dispatch<SetStateAction<number | undefined>>;
}

const Navbar = (props: Props) => {
  return (
    <nav className={`navbar ${styles.navbar}`}>
      <div className="container-lg">
        <div className="d-flex flex-column flex-md-row align-items-center justify-content-between w-100">
          <div className="me-0 me-md-4 mt-2 mt-md-0 w-100">
            <div className="d-flex flex-row align-items-start w-100">
              <Link to="/" onClick={() => props.setScrollPosition(0)} className="cursorPointer">
                <img className={styles.logo} alt="CLOMonitor logo" src={logo} />
              </Link>
              <div className={`ms-1 badge rounded-0 text-uppercase ${styles.badge} alphaBadge`}>Alpha</div>
              <MobileSettings />
            </div>
          </div>
          <Searchbar classNameWrapper="my-3 w-100" setScrollPosition={props.setScrollPosition} />
          <div className={`d-none d-md-flex flex-row align-items-center ms-auto ${styles.searchWrapper}`}>
            <Link
              to="/stats"
              className={`position-relative ms-3 text-light text-uppercase fw-bold text-decoration-none ${styles.link} navbarLink`}
            >
              Stats
            </Link>
            <ThemeSwitch />
          </div>
        </div>
      </div>
    </nav>
  );
};

export default Navbar;
