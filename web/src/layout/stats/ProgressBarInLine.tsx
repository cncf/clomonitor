import { getCategoryColor } from 'clo-ui/utils/getCategoryColor';

import { ReportOption } from '../../types';
import styles from './ProgressBarInLine.module.css';

interface Props {
  title: string;
  icon: JSX.Element;
  value: number;
  name: ReportOption;
  onSelectCheck: (name: ReportOption) => void;
}

const ProgressBarInLine = (props: Props) => {
  const color = getCategoryColor(props.value);

  return (
    <div className={`d-flex flex-column ${styles.wrapper}`}>
      <div className={`d-flex flex-row align-items-center ${styles.progressWrapper}`}>
        <div className={`me-1 me-md-2 position-relative ${styles.icon}`}>{props.icon}</div>
        <div className={styles.progressTitle}>
          <span className="d-inline-block d-md-none">{props.title}</span>
          <span className="d-none d-md-inline-block">
            <button
              aria-label={`Search projects with passed ${props.title} check`}
              onClick={() => props.onSelectCheck(props.name)}
              className={`btn btn-link p-0 ${styles.btn}`}
            >
              {props.title}
            </button>
          </span>
        </div>
        <div className="flex-grow-1 ms-2">
          <div className={`progress rounded-0 ${styles.progress}`}>
            <div
              className={`progress-bar ${styles.progressbar}`}
              role="progressbar"
              style={{ width: `${props.value || 1}%`, backgroundColor: `var(--clo-${color})` }}
            />
          </div>
        </div>
        <div className={`ps-2 lh-1 text-end fw-bold lightText ${styles.scoreWrapper}`}>{props.value}%</div>
      </div>
    </div>
  );
};

export default ProgressBarInLine;
