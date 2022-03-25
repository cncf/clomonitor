import { CATEGORY_ICONS } from '../../data';
import { ScoreType } from '../../types';
import styles from './Average.module.css';
import ProgressBar from './ProgressBar';

interface Props {
  title: string;
  data: { [key in ScoreType]: number };
}

const Average = (props: Props) => {
  return (
    <>
      <div className={`card-header fw-bold text-uppercase text-center ${styles.cardHeader}`}>{props.title}</div>
      <div className="card-body pt-3 px-4 pb-0">
        <ProgressBar
          title="Documentation"
          icon={CATEGORY_ICONS[ScoreType.Documentation]}
          value={props.data.documentation}
        />
        <ProgressBar title="License" icon={CATEGORY_ICONS[ScoreType.License]} value={props.data.license} />
        <ProgressBar
          title="Best Practices"
          icon={CATEGORY_ICONS[ScoreType.BestPractices]}
          value={props.data.best_practices}
        />
        <ProgressBar title="Security" icon={CATEGORY_ICONS[ScoreType.Security]} value={props.data.security} />
        <ProgressBar title="Legal" icon={CATEGORY_ICONS[ScoreType.Legal]} value={props.data.legal} />
      </div>
    </>
  );
};

export default Average;
