import classNames from 'classnames';
import { isUndefined } from 'lodash';
import { useEffect, useRef, useState } from 'react';

import useOutsideClick from '../../hooks/useOutsideClick';
import styles from './DropdownOnHover.module.css';

interface Props {
  linkContent: JSX.Element | string;
  children: JSX.Element;
  className?: string;
}

const DropdownOnHover = (props: Props) => {
  const ref = useRef(null);
  const [openStatus, setOpenStatus] = useState(false);
  const [onLinkHover, setOnLinkHover] = useState(false);
  const [onDropdownHover, setOnDropdownHover] = useState(false);
  useOutsideClick([ref], openStatus, () => setOpenStatus(false));

  useEffect(() => {
    let timeout: NodeJS.Timeout;
    if (!openStatus && (onLinkHover || onDropdownHover)) {
      timeout = setTimeout(() => {
        setOpenStatus(true);
      }, 100);
    }
    if (openStatus && !onLinkHover && !onDropdownHover) {
      timeout = setTimeout(() => {
        // Delay to hide the dropdown to let some time for changing between dropdown and link
        setOpenStatus(false);
      }, 50);
    }
    return () => {
      if (!isUndefined(timeout)) {
        clearTimeout(timeout);
      }
    };
  }, [onLinkHover, onDropdownHover, openStatus]);

  return (
    <>
      <div className={props.className}>
        <div className="position-absolute">
          <div
            ref={ref}
            role="complementary"
            className={classNames('dropdown-menu rounded-0 text-wrap', styles.dropdown, {
              show: openStatus,
            })}
            onMouseEnter={() => setOnDropdownHover(true)}
            onMouseLeave={() => setOnDropdownHover(false)}
          >
            <div className="px-3 py-1">{props.children}</div>
          </div>
        </div>

        <div
          onMouseEnter={(e) => {
            e.preventDefault();
            setOnLinkHover(true);
          }}
          onMouseLeave={() => {
            setOnLinkHover(false);
          }}
          aria-expanded={openStatus}
        >
          {props.linkContent}
        </div>
      </div>
    </>
  );
};

export default DropdownOnHover;
