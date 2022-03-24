import { Dispatch, SetStateAction } from 'react';
import { Outlet } from 'react-router-dom';

import useOnLocationChange from '../hooks/useOnLocationChange';
import updateMetaIndex from '../utils/updateMetaIndex';
import Footer from './navigation/Footer';
import Navbar from './navigation/Navbar';

interface Props {
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
      <Outlet />
      <Footer />
    </div>
  );
};

export default Layout;
