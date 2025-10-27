import classNames from 'classnames';
import { Foundation } from 'clo-ui/components/Foundation';
import { useOutsideClick } from 'clo-ui/hooks/useOutsideClick';
import { MouseEvent as ReactMouseEvent, useRef, useState } from 'react';
import { VscThreeBars } from 'react-icons/vsc';

import BadgeModal from './BadgeModal';
import styles from './ProjectDropdown.module.css';
import ReportSummaryModal from './ReportSummaryModal';

interface OpenModalStatus {
  status: boolean;
  name?: string;
}

enum Modals {
  Badge = 'badge',
  ReportSummary = 'reportSummary',
}

interface Props {
  foundation: Foundation;
  projectName: string;
  projectDisplayName?: string;
}

const ProjectDropdown = (props: Props) => {
  const ref = useRef(null);
  const [visibleDropdown, setVisibleDropdown] = useState<boolean>(false);
  const [openStatus, setOpenStatus] = useState<OpenModalStatus>({ status: false });
  useOutsideClick([ref], visibleDropdown, () => setVisibleDropdown(false));

  const onCloseModal = () => {
    setOpenStatus({ status: false });
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
          aria-label={`Open project options for ${props.projectDisplayName || props.projectName}`}
          aria-expanded={visibleDropdown}
          aria-haspopup="true"
        >
          <VscThreeBars />
        </button>

        <ul
          role="complementary"
          className={classNames('dropdown-menu rounded-0', styles.dropdown, { show: visibleDropdown })}
        >
          <li>
            <button
              className="dropdown-item"
              onClick={(e) => {
                e.stopPropagation();
                e.preventDefault();

                setVisibleDropdown(false);
                setOpenStatus({ name: Modals.Badge, status: true });
              }}
            >
              Get badge
            </button>
          </li>
          <li>
            <button
              className="dropdown-item"
              onClick={(e) => {
                e.stopPropagation();
                e.preventDefault();

                setVisibleDropdown(false);
                setOpenStatus({ name: Modals.ReportSummary, status: true });
              }}
            >
              Embed report summary
            </button>
          </li>
        </ul>
      </div>

      <BadgeModal
        foundation={props.foundation}
        projectName={props.projectName}
        openStatus={openStatus}
        onCloseModal={onCloseModal}
      />

      <ReportSummaryModal
        foundation={props.foundation}
        projectName={props.projectName}
        openStatus={openStatus}
        onCloseModal={onCloseModal}
      />
    </>
  );
};

export default ProjectDropdown;
