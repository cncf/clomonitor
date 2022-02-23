import { MdCategory } from 'react-icons/md';

import { Category } from '../../../types';
import styles from './Badge.module.css';

interface Props {
  categoryId: Category;
  className?: string;
}

const CartegoryBadge = (props: Props) => {
  return (
    <div
      data-testid="category-badge"
      className={`badge text-secondary border border-secondary rounded-0 position-relative ${styles.badge} ${props.className}`}
    >
      <div className="d-flex flex-row align-items-center">
        <MdCategory className="me-2" />
        {Category[props.categoryId]}
      </div>
    </div>
  );
};

export default CartegoryBadge;
