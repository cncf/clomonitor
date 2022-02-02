import getCategoryColor from '../../../utils/getCategoryColor';
import styles from './Badge.module.css';

interface Props {
  value: number;
}

const Badge = (props: Props) => {
  const color = getCategoryColor(props.value);
  return (
    <div
      className={`mx-auto px-2 text-center ${styles.badge}`}
      style={{
        borderBottomColor: `var(--rm-${color})`,
      }}
    >
      <span className="text-dark fw-bold">{props.value}</span>
    </div>
  );
};

export default Badge;
