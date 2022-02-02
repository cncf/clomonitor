import { REPORT_OPTIONS_BY_CATEGORY } from '../../../data';
import { ReportOption, ScoreType } from '../../../types';
import getCategoryColor from '../../../utils/getCategoryColor';
import OptionBox from './OptionBox';
import styles from './Row.module.css';
import Title from './Title';

interface Props {
  reportId: string;
  name: ScoreType;
  label: string;
  icon: JSX.Element;
  data: {
    [key: string]: string | boolean;
  };
  score: number;
}

const Row = (props: Props) => {
  const color = getCategoryColor(props.score);

  return (
    <div className="p-4 border border-top-0">
      <div className="mx-1">
        <Title title={props.label} icon={props.icon} />
        <div className="row mt-2 mb-4 align-items-center">
          <div className="col-9 col-md-6">
            <div className={`progress rounded-0 ${styles.progress}`}>
              <div
                className="progress-bar progress-bar-striped"
                role="progressbar"
                style={{ width: `${props.score || 1}%`, backgroundColor: `var(--rm-${color})` }}
              />
            </div>
          </div>
          <div className="col">
            <small className="fw-bold">{props.score}%</small>
          </div>
        </div>
        <div className={`d-flex flex-row align-items-center flex-wrap ${styles.boxWrapper}`}>
          {(REPORT_OPTIONS_BY_CATEGORY as any)[props.name].map((opt: string) => {
            return (
              <OptionBox
                key={`${props.reportId}_${props.label}_${opt}`}
                label={opt as ReportOption}
                value={props.data[opt]}
              />
            );
          })}
        </div>
      </div>
    </div>
  );
};

export default Row;
