import { FaGithub } from 'react-icons/fa';
import { FiExternalLink } from 'react-icons/fi';

import logo from '../../media/remonitor.svg';
import ExternalLink from '../common/ExternalLink';
import styles from './Footer.module.css';

const Footer = () => {
  return (
    <footer className={`py-5 ${styles.footer}`}>
      <div className="container-lg">
        <div className="d-flex flex-row flex-wrap align-items-stretch justify-content-between text-light">
          <div className={styles.footerCol}>
            <div className="h6 fw-bold text-uppercase">Project</div>
            <div className="d-flex flex-column text-start">
              <ExternalLink
                className="mb-1 opacity-75"
                href="/"
                label="Open documentation"
                target="_self"
                disabled
                btnType
              >
                Documentation
              </ExternalLink>
            </div>
          </div>

          <div className={styles.footerCol}>
            <div className="h6 fw-bold text-uppercase">Community</div>
            <div className="d-flex flex-column text-start">
              <ExternalLink
                className="mb-1 opacity-75"
                href="https://github.com/cncf/hub"
                label="Open Github"
                disabled
                btnType
              >
                <div className="d-flex align-items-center">
                  <FaGithub className="me-2" />
                  GitHub
                </div>
              </ExternalLink>
            </div>
          </div>

          <div className={styles.footerCol}>
            <div className="h6 fw-bold text-uppercase">About</div>
            <div className={`opacity-75 ${styles.license}`}>
              Re-Monitor is an <b className="d-inline-block">Open Source</b> project licensed under the{' '}
              <ExternalLink
                className="d-inline-block mb-1"
                href="https://www.apache.org/licenses/LICENSE-2.0"
                label="Open Apache License 2.0 documentation"
              >
                <div className="d-flex align-items-center">
                  Apache License 2.0
                  <FiExternalLink className={`ms-1 ${styles.miniIcon}`} />
                </div>
              </ExternalLink>
            </div>
          </div>

          <div className={`ms-0 ms-lg-auto ${styles.fullMobileSection}`}>
            <div className="d-flex flex-column align-items-center justify-content-center h-100">
              <img className={styles.logo} alt="Logo" src={logo} />
            </div>
          </div>
        </div>
      </div>
    </footer>
  );
};

export default Footer;
