import { ExternalLink } from 'clo-ui/components/ExternalLink';
import { Navbar as NavbarWrapper } from 'clo-ui/components/Navbar';
import { Dispatch, SetStateAction } from 'react';
import { Link } from 'react-router-dom';

import logo from '../../media/clomonitor.svg';
import MobileSettings from './MobileSettings';
import styles from './Navbar.module.css';
import Searchbar from './Searchbar';
import Settings from './Settings';

interface Props {
  setScrollPosition: Dispatch<SetStateAction<number | undefined>>;
}

const Navbar = (props: Props) => {
  return (
    <NavbarWrapper navbarClassname={styles.navbar}>
      <>
        <div className={`me-0 me-md-4 mt-2 mt-md-0 ${styles.line}`}>
          <div className="d-flex flex-row align-items-start">
            <Link to="/" onClick={() => props.setScrollPosition(0)} className="cursorPointer">
              <img className={styles.logo} alt="CLOMonitor logo" src={logo} />
            </Link>
            <MobileSettings />
          </div>
        </div>
        <Searchbar classNameWrapper={`my-3 ${styles.line}`} setScrollPosition={props.setScrollPosition} />
        <div className={`d-none d-md-flex flex-row align-items-center ms-auto ${styles.searchWrapper}`}>
          <ExternalLink
            className={`position-relative ms-3 text-light text-uppercase fw-bold text-decoration-none ${styles.link} navbarLink`}
            href="/docs"
            label="Open documentation"
            target="_self"
          >
            Docs
          </ExternalLink>
          <Link
            to="/stats"
            className={`position-relative ms-4 text-light text-uppercase fw-bold text-decoration-none ${styles.link} navbarLink`}
          >
            Stats
          </Link>
          <Settings />
        </div>
      </>
    </NavbarWrapper>
  );
};

export default Navbar;
