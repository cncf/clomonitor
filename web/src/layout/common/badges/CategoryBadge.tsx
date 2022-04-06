import { MdCategory } from 'react-icons/md';

import styles from './Badge.module.css';

interface Props {
  category: string;
  className?: string;
}

const CartegoryBadge = (props: Props) => {
  return (
    <div
      data-testid="category-badge"
      className={`badge text-dark lighterText rounded-0 position-relative ${styles.badge} ${props.className}`}
    >
      <div className="d-flex flex-row align-items-center text-capitalize">
        <MdCategory className="me-2" />
        {props.category}
      </div>
    </div>
  );
};

export default CartegoryBadge;
