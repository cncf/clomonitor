import { GiStairsGoal } from 'react-icons/gi';

import { Maturity } from '../../../types';
import styles from './Badge.module.css';

interface Props {
  maturityLevel: Maturity;
  className?: string;
}

const MaturityBadge = (props: Props) => {
  return (
    <div
      data-testid="maturity-badge"
      className={`badge text-secondary border border-secondary rounded-0 position-relative ${styles.badge} ${props.className}`}
    >
      <div className="d-flex flex-row align-items-center">
        <GiStairsGoal className="me-2" />
        {Maturity[props.maturityLevel]}
      </div>
    </div>
  );
};

export default MaturityBadge;
