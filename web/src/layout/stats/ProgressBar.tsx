import { getCategoryColor } from 'clo-ui/utils/getCategoryColor';

import styles from './ProgressBar.module.css';

interface Props {
  title: string;
  icon: JSX.Element;
  value: number;
}

const ProgressBar = (props: Props) => {
  const color = getCategoryColor(props.value);
  const clampedValue = Math.max(0, Math.min(100, props.value));
  const barWidth = clampedValue === 0 ? 1 : clampedValue;

  return (
    <div className="d-flex flex-column">
      <div className={`d-flex flex-row align-items-center mb-1 mb-md-0 ${styles.progressTitle}`}>
        <div className={`me-2 position-relative ${styles.icon}`}>{props.icon}</div>
        <div>{props.title}</div>
      </div>
      <div className="d-flex flex-row mb-3 align-items-center">
        <div className={`flex-grow-1 ${styles.progressbarWrapper}`}>
          <div className={`progress rounded-0 ${styles.progress}`}>
            <div
              className={`progress-bar ${styles.progressbar}`}
              role="progressbar"
              style={{ width: `${barWidth}%`, backgroundColor: `var(--clo-${color})` }}
              aria-valuenow={clampedValue}
              aria-valuemin={0}
              aria-valuemax={100}
              aria-label={`${props.title} passed checks percentage`}
            />
          </div>
        </div>
        <div className={`ps-1 ps-md-3 lh-1 text-end ${styles.scoreWrapper}`}>
          <small className="fw-bold lightText">{props.value}%</small>
        </div>
      </div>
    </div>
  );
};

export default ProgressBar;
