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
      style={{ borderColor: `var(--rm-${color})` }}
      className={`d-flex align-items-center justify-content-center rounded-pill ${styles.value} global ${props.className}`}
    >
      {props.score}
    </div>
  );
};

export default RoundScore;
