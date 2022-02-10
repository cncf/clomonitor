import { Dispatch, SetStateAction } from 'react';
import { Outlet } from 'react-router-dom';

import Footer from './navigation/Footer';
import Navbar from './navigation/Navbar';

interface Props {
  setScrollPosition: Dispatch<SetStateAction<number | undefined>>;
}

const Layout = (props: Props) => {
  return (
    <div className="h-100 d-flex flex-column">
      <Navbar setScrollPosition={props.setScrollPosition} />
      <Outlet />
      <Footer />
    </div>
  );
};

export default Layout;
