import { SECTIONS } from '../../data';
import { ScoreType, SectionInfo } from '../../types';
import styles from './Average.module.css';
import ProgressBar from './ProgressBar';

interface Props {
  title: string;
  data: { [key in ScoreType]?: number | null };
}

const Average = (props: Props) => {
  return (
    <>
      <div className={`card-header fw-bold text-uppercase text-center ${styles.cardHeader}`}>{props.title}</div>
      <div className="card-body pt-2 pt-md-3 px-3 px-md-4 pb-0">
        {SECTIONS.map((section: SectionInfo) => {
          const value = props.data[section.type];
          if (value === undefined || value === null) return null;
          return <ProgressBar key={`average_${section.type}`} title={section.name} icon={section.icon} value={value} />;
        })}
      </div>
    </>
  );
};

export default Average;
