import classNames from 'classnames';
import { MouseEvent as ReactMouseEvent, useRef, useState } from 'react';
import { GoThreeBars } from 'react-icons/go';
import { useParams } from 'react-router-dom';

import useOutsideClick from '../../../hooks/useOutsideClick';
import ExternalLink from '../../common/ExternalLink';
import styles from './RepositoryDropdown.module.css';

interface Props {
  repoName: string;
}

const RepositoryDropdown = (props: Props) => {
  const { project, foundation } = useParams();
  const ref = useRef(null);
  const [visibleDropdown, setVisibleDropdown] = useState<boolean>(false);
  useOutsideClick([ref], visibleDropdown, () => setVisibleDropdown(false));

  return (
    <>
      <div ref={ref} className="ms-auto position-relative">
        <button
          data-testid="dropdown-btn"
          type="button"
          className={`btn btn-sm btn-primary text-white rounded-0 lh-1 ${styles.btn}`}
          onClick={(e: ReactMouseEvent<HTMLButtonElement, MouseEvent>) => {
            e.preventDefault();
            e.stopPropagation();
            setVisibleDropdown(!visibleDropdown);
          }}
        >
          <GoThreeBars />
        </button>

        <ul
          role="complementary"
          className={classNames('dropdown-menu rounded-0', styles.dropdown, { show: visibleDropdown })}
        >
          <li>
            <ExternalLink
              href={`/api/projects/${foundation}/${project}/${props.repoName}/report.md`}
              className="dropdown-item lightText"
              label="Open repository report"
              target="_self"
              onClick={() => setVisibleDropdown(false)}
            >
              <div>Get markdown</div>
            </ExternalLink>
          </li>
        </ul>
      </div>
    </>
  );
};

export default RepositoryDropdown;
