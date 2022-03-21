import getCategoryColor from '../../utils/getCategoryColor';
import styles from './ProgressBarInLine.module.css';

interface Props {
  title: string;
  icon: JSX.Element;
  value: number;
}

const ProgressBarInLine = (props: Props) => {
  const color = getCategoryColor(props.value);

  return (
    <div className={`d-flex flex-column ${styles.wrapper}`}>
      <div className={`d-flex flex-row align-items-center ${styles.progressWrapper}`}>
        <div className={`me-2 position-relative ${styles.icon}`}>{props.icon}</div>
        <div className={styles.progressTitle}>{props.title}</div>
        <div className="flex-grow-1 ms-2">
          <div className={`progress rounded-0 ${styles.progress}`}>
            <div
              className="progress-bar"
              role="progressbar"
              style={{ width: `${props.value || 1}%`, backgroundColor: `var(--rm-${color})` }}
            />
          </div>
        </div>
        <div className={`ps-2 lh-1 text-end fw-bold lightText ${styles.scoreWrapper}`}>{props.value}%</div>
      </div>
    </div>
  );
};

export default ProgressBarInLine;
