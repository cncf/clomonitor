import { FOUNDATIONS } from '../../../data';
import { Foundation } from '../../../types';
import styles from './Badge.module.css';

interface Props {
  foundation: Foundation;
  className?: string;
}

const FoundationBadge = (props: Props) => {
  const foundationData = FOUNDATIONS[props.foundation];

  return (
    <div
      data-testid="foundation-badge"
      className={`badge text-light extraLightText rounded-0 position-relative ${styles.badge} ${styles.blueBadge} ${props.className}`}
    >
      <div className="d-flex flex-row align-items-center text-uppercase">{foundationData.name}</div>
    </div>
  );
};

export default FoundationBadge;
