import { Dispatch, SetStateAction } from 'react';
import { Link } from 'react-router-dom';

import logo from '../../media/clomonitor.svg';
import ThemeSwitch from '../common/ThemeSwitch';
import Searchbar from '../navigation/Searchbar';
import styles from './Navbar.module.css';

interface Props {
  setScrollPosition: Dispatch<SetStateAction<number | undefined>>;
}

const Navbar = (props: Props) => {
  return (
    <nav className={`navbar ${styles.navbar}`}>
      <div className="container-lg">
        <div className="d-flex flex-column flex-md-row align-items-center justify-content-between w-100">
          <div className="me-0 me-md-4 mt-2 mt-md-0">
            <div className="d-flex flex-row align-items-start">
              <Link to="/" onClick={() => props.setScrollPosition(0)} className="cursorPointer">
                <img className={styles.logo} alt="CLOMonitor logo" src={logo} />
              </Link>
              <div className={`ms-1 badge rounded-0 text-uppercase ${styles.badge} alphaBadge`}>Alpha</div>
            </div>
          </div>
          <div className={`d-flex flex-row align-items-center ${styles.searchWrapper}`}>
            <Searchbar classNameWrapper="my-3 flex-grow-1" setScrollPosition={props.setScrollPosition} />
            <ThemeSwitch />
          </div>
        </div>
      </div>
    </nav>
  );
};

export default Navbar;
