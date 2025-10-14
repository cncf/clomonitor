import classNames from 'classnames';
import { useOutsideClick } from 'clo-ui/hooks/useOutsideClick';
import { MouseEvent as ReactMouseEvent, useRef, useState } from 'react';
import { VscThreeBars } from 'react-icons/vsc';

import styles from './RepositoryDropdown.module.css';
import RepositoryReportModal from './RepositoryReportModal';

interface Props {
  repoName: string;
}

const RepositoryDropdown = (props: Props) => {
  const ref = useRef(null);
  const [visibleDropdown, setVisibleDropdown] = useState<boolean>(false);
  const [openReportModalStatus, setOpenReportModalStatus] = useState<boolean>(false);
  useOutsideClick([ref], visibleDropdown, () => setVisibleDropdown(false));

  const onOpenModal = () => {
    setOpenReportModalStatus(true);
    setVisibleDropdown(false);
  };

  const onCloseModal = () => {
    setOpenReportModalStatus(false);
  };

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
          <VscThreeBars />
        </button>

        <ul
          role="complementary"
          className={classNames('dropdown-menu rounded-0', styles.dropdown, { show: visibleDropdown })}
        >
          <li>
            <button className="dropdown-item lightText" aria-label="Open repository report" onClick={onOpenModal}>
              Get markdown
            </button>
          </li>
        </ul>
      </div>

      <RepositoryReportModal openStatus={openReportModalStatus} repoName={props.repoName} onCloseModal={onCloseModal} />
    </>
  );
};

export default RepositoryDropdown;
