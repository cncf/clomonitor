import { TiWarningOutline } from 'react-icons/ti';
import { Link } from 'react-router-dom';

import styles from './NotFound.module.css';

const NotFound = () => {
  return (
    <main
      role="main"
      className="container-lg flex-grow-1 p-5 d-flex flex-column align-items-center justify-content-center"
    >
      <TiWarningOutline className={`m-3 ${styles.icon}`} />
      <div className="h1 text-center mb-4">Error 404 - Page Not Found</div>
      <div className="h3 text-center mb-5">The page you were looking for wasn't found</div>
      <Link to="/search?page=1" className="btn btn-primary rounded-0 text-white btn-lg text-decoration-none">
        Back Search
      </Link>
    </main>
  );
};

export default NotFound;
