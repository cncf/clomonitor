import getCategoryColor from '../../utils/getCategoryColor';
import styles from './RoundScore.module.css';

interface Props {
  score: number;
  className?: string;
}

const RoundScore = (props: Props) => {
  const color = getCategoryColor(props.score);

  return (
    <div
      data-testid="global-score"
      className={`d-flex align-items-center justify-content-center rounded-pill ${styles.score} ${props.className}`}
    >
      <div className={styles.chart}>
        <svg viewBox="0 0 36 36" className="d-block">
          <path
            className={styles.circleBg}
            d="M18 2.0845
          a 15.9155 15.9155 0 0 1 0 31.831
          a 15.9155 15.9155 0 0 1 0 -31.831"
          />
          <path
            className={styles.circle}
            strokeDasharray={`${props.score}, 100`}
            style={{ stroke: `var(--rm-${color})` }}
            d="M18 2.0845
          a 15.9155 15.9155 0 0 1 0 31.831
          a 15.9155 15.9155 0 0 1 0 -31.831"
          />
          <text x="18" y="23" className={styles.value}>
            {props.score}
          </text>
        </svg>
      </div>
    </div>
  );
};

export default RoundScore;
