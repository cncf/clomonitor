import classNames from 'classnames';
import { ExternalLink } from 'clo-ui/components/ExternalLink';
import { Footer as FooterWrapper } from 'clo-ui/components/Footer';
import { FaGithub } from 'react-icons/fa';
import { FiExternalLink } from 'react-icons/fi';
import { Link } from 'react-router-dom';
import styles from './Footer.module.css';

const logo = new URL('../../media/clomonitor.svg', import.meta.url).pathname;

interface Props {
  invisibleFooter: boolean;
}

const Footer = (props: Props) => {
  return (
    <FooterWrapper
      className={classNames('py-5', { 'opacity-0': props.invisibleFooter })}
      logo={<img className={styles.logo} alt="Logo" src={logo} />}
    >
      <>
        <div className={styles.footerCol}>
          <div className="h6 fw-bold text-uppercase">Project</div>
          <div className="d-flex flex-column text-start">
            <ExternalLink className="mb-1 opacity-75" href="/docs" label="Open documentation" target="_self">
              Documentation
            </ExternalLink>
            <Link className="mb-1 opacity-75" to="/stats">
              Statistics
            </Link>
          </div>
        </div>

        <div className={styles.footerCol}>
          <div className="h6 fw-bold text-uppercase">Community</div>
          <div className="d-flex flex-column text-start">
            <ExternalLink className="mb-1 opacity-75" href="https://github.com/cncf/clomonitor" label="Open Github">
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
            CLOMonitor is an <b className="d-inline-block">Open Source</b> project licensed under the{' '}
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
      </>
    </FooterWrapper>
  );
};

export default Footer;
