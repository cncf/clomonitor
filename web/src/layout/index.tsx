import { Outlet } from 'react-router-dom';

import Footer from './navigation/Footer';
import Navbar from './navigation/Navbar';

const Layout = () => {
  return (
    <div className="h-100 d-flex flex-column">
      <Navbar />
      <Outlet />
      <Footer />
    </div>
  );
};

export default Layout;
