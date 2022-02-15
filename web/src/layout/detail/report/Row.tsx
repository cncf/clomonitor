import { isUndefined } from 'lodash';

import { REPORT_OPTIONS_BY_CATEGORY } from '../../../data';
import { ReportOption, RepositoryKind, ScoreType } from '../../../types';
import getCategoryColor from '../../../utils/getCategoryColor';
import OptionCell from './OptionCell';
import styles from './Row.module.css';
import Title from './Title';

interface Props {
  reportId: string;
  repoKind: RepositoryKind;
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
  const options: ReportOption[] = (REPORT_OPTIONS_BY_CATEGORY as any)[props.repoKind][props.name];

  if (isUndefined(options) || options.length === 0) return null;

  return (
    <div className="p-3 p-md-4 border border-top-0">
      <div className="mx-0 mx-md-1">
        <Title title={props.label} icon={props.icon} />
        <div className="d-flex flex-row mt-2 mb-4 align-items-center">
          <div className={`flex-grow-1 ${styles.progressbarWrapper}`}>
            <div className={`progress rounded-0 ${styles.progress}`}>
              <div
                className="progress-bar progress-bar-striped"
                role="progressbar"
                style={{ width: `${props.score || 1}%`, backgroundColor: `var(--rm-${color})` }}
              />
            </div>
          </div>
          <div className={`ps-3 lh-1 ${styles.scoreWrapper}`}>
            <small className="fw-bold">{props.score}%</small>
          </div>
        </div>
        <div>
          <table className={`table align-middle w-100 border ${styles.table}`}>
            <tbody>
              {options.map((opt: string) => {
                return (
                  <OptionCell
                    key={`${props.reportId}_${props.label}_${opt}_cell`}
                    repoKind={props.repoKind}
                    label={opt as ReportOption}
                    value={props.data[opt]}
                  />
                );
              })}
            </tbody>
          </table>
        </div>
      </div>
    </div>
  );
};

export default Row;
