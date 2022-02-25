import isUndefined from 'lodash/isUndefined';
import { FiExternalLink } from 'react-icons/fi';

import styles from './ExternalLink.module.css';

interface Props {
  children: JSX.Element | JSX.Element[] | string;
  href: string;
  className?: string;
  btnType?: boolean;
  target?: string;
  label?: string;
  ariaHidden?: boolean;
  visibleExternalIcon?: boolean;
  disabled?: boolean;
}

const ExternalLink = (props: Props) => {
  const getData = () => (
    <div className="d-flex flex-row align-items-baseline">
      {props.children}
      {!isUndefined(props.visibleExternalIcon) && props.visibleExternalIcon && (
        <FiExternalLink className={`ms-2 ${styles.icon}`} />
      )}
    </div>
  );

  return (
    <>
      {!isUndefined(props.btnType) && props.btnType ? (
        <button
          type="button"
          className={`btn p-0 ${styles.link} ${props.className}`}
          onClick={(e) => {
            e.stopPropagation();
            e.preventDefault();

            if (isUndefined(props.disabled) || !props.disabled) {
              window.open(props.href, props.target || '_blank');
            }
          }}
          aria-label={props.label || 'Open external link'}
          aria-hidden={props.ariaHidden}
          tabIndex={-1}
        >
          {getData()}
        </button>
      ) : (
        <a
          className={`${styles.link} ${props.className}`}
          href={props.href}
          target={props.target || '_blank'}
          rel="noopener noreferrer"
          onClick={(e) => e.stopPropagation()}
          aria-label={props.label || 'Open external link'}
          aria-hidden={props.ariaHidden}
          tabIndex={-1}
        >
          {getData()}
        </a>
      )}
    </>
  );
};

export default ExternalLink;
