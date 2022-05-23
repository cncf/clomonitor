import { Dispatch, SetStateAction } from 'react';
import { Outlet } from 'react-router-dom';

import useOnLocationChange from '../hooks/useOnLocationChange';
import updateMetaIndex from '../utils/updateMetaIndex';
import styles from './Layout.module.css';
import Footer from './navigation/Footer';
import Navbar from './navigation/Navbar';

interface Props {
  invisibleFooter: boolean;
  setScrollPosition: Dispatch<SetStateAction<number | undefined>>;
}

const Layout = (props: Props) => {
  useOnLocationChange((loc: Location) => {
    if (!loc.pathname.startsWith('/projects')) {
      updateMetaIndex();
    }
  });

  return (
    <div className="h-100 d-flex flex-column">
      <Navbar setScrollPosition={props.setScrollPosition} />
      <div className={`d-flex flex-column flex-grow-1 ${styles.wrapper}`}>
        <Outlet />
      </div>
      <Footer invisibleFooter={props.invisibleFooter} />
    </div>
  );
};

export default Layout;
