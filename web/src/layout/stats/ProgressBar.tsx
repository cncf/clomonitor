import { getCategoryColor } from 'clo-ui/utils/getCategoryColor';

import styles from './ProgressBar.module.css';

interface Props {
  title: string;
  icon: JSX.Element;
  value: number;
}

const ProgressBar = (props: Props) => {
  const color = getCategoryColor(props.value);

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
              style={{ width: `${props.value || 1}%`, backgroundColor: `var(--clo-${color})` }}
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
