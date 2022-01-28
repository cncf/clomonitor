import classNames from 'classnames';
import { MouseEvent as ReactMouseEvent, useRef, useState } from 'react';
import { GoThreeBars } from 'react-icons/go';

import useOutsideClick from '../../hooks/useOutsideClick';
import Modal from '../common/Modal';
import styles from './Dropdown.module.css';

interface openModalStatus {
  status: boolean;
  name?: string;
}

enum Modals {
  Badge = 'badge',
  Embed = 'embed',
}

const Dropdown = () => {
  const ref = useRef(null);
  const [visibleDropdown, setVisibleDropdown] = useState<boolean>(false);
  const [openStatus, setOpenStatus] = useState<openModalStatus>({ status: false });
  useOutsideClick([ref], visibleDropdown, () => setVisibleDropdown(false));

  const onCloseModal = () => {
    setOpenStatus({ status: false });
  };

  return (
    <>
      <div ref={ref} className="ms-auto position-relative">
        <button
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

        <ul className={classNames('dropdown-menu rounded-0', styles.dropdown, { show: visibleDropdown })}>
          <li>
            <button
              className="dropdown-item"
              onClick={(e) => {
                e.stopPropagation();
                e.preventDefault();

                setVisibleDropdown(false);
                // setOpenStatus({ name: Modals.Embed, status: true });
              }}
            >
              Embed quality report
            </button>
          </li>
          <li>
            <button
              className="dropdown-item"
              onClick={(e) => {
                e.stopPropagation();
                e.preventDefault();

                setVisibleDropdown(false);
                // setOpenStatus({ name: Modals.Badge, status: true });
              }}
            >
              Get badge
            </button>
          </li>
        </ul>
      </div>

      <Modal header="Badge" onClose={onCloseModal} open={openStatus.status && openStatus.name === Modals.Badge}>
        <div>kjhskdjf</div>
      </Modal>

      <Modal
        header="Quality report"
        onClose={onCloseModal}
        open={openStatus.status && openStatus.name === Modals.Embed}
      >
        <div className="w-100 position-relative">
          <div className="mt-4 mb-3">
            <div className="form-check form-switch ps-0">
              <label htmlFor="header" className={`form-check-label fw-bold ${styles.label}`}>
                Header
              </label>{' '}
              <input
                id="header"
                type="checkbox"
                className="form-check-input position-absolute ms-2"
                value="true"
                role="switch"
                onChange={() => console.log('change')}
                checked
              />
            </div>

            <div className="form-text text-muted mt-2">Lorem ipsum...</div>
          </div>

          <div className="mt-3 mb-2">
            <label className={`form-label fw-bold ${styles.label}`}>Code</label>
          </div>
        </div>
      </Modal>
    </>
  );
};

export default Dropdown;
