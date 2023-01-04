import { ElementType } from 'react';
import { GoLink } from 'react-icons/go';
import { useLocation, useNavigate } from 'react-router-dom';

import getAnchorValue from '../../utils/getAnchorValue';
import styles from './AnchorHeader.module.css';

interface Props {
  className?: string;
  title: string;
  scrollIntoView: (id?: string) => void;
}

const AnchorHeader: ElementType = (props: Props) => {
  const navigate = useNavigate();
  const location = useLocation();
  const anchor = getAnchorValue(props.title);

  const goToAnchor = () => {
    props.scrollIntoView(`#${anchor}`);
    navigate(
      {
        pathname: location.pathname,
        hash: anchor,
      },
      { state: location.state }
    );
  };

  return (
    <div className={`position-relative ${styles.header}`}>
      <div className={`position-absolute ${styles.headerAnchor}`} id={anchor} />

      <div className={`${styles.headingWrapper} ${props.className}`}>
        {props.title}
        <button
          className={`btn btn-link ${styles.headingLink}`}
          onClick={goToAnchor}
          aria-label={`Go to ${props.title}`}
        >
          <GoLink />
        </button>
      </div>
    </div>
  );
};

export default AnchorHeader;
