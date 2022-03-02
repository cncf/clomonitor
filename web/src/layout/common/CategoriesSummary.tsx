import classNames from 'classnames';

import { CATEGORY_ICONS } from '../../data';
import { Score, ScoreType } from '../../types';
import styles from './CategoriesSummary.module.css';
import CategoryProgressbar from './CategoryProgressbar';
import RoundScore from './RoundScore';

interface Props {
  score: Score;
  bigSize: boolean;
}

const CategoriesSummary = (props: Props) => {
  return (
    <div
      className={classNames(
        'align-items-center d-flex flex-column flex-md-row',
        styles.summary,
        {
          'flex-lg-column flex-xl-row': !props.bigSize,
        },
        { [styles.bigSize]: props.bigSize }
      )}
    >
      <div
        className={classNames(
          'd-none d-md-block',
          { 'd-lg-none d-xl-block d-lg-none d-xl-block': !props.bigSize },
          { 'mx-3': props.bigSize }
        )}
      >
        <div className="d-flex flex-column me-0 me-sm-4 mb-2 mb-sm-0">
          <RoundScore score={props.score.global} />
        </div>
      </div>

      <div
        className={classNames('flex-grow-1 w-100 position-relative', {
          'px-0 px-sm-3': props.bigSize,
        })}
      >
        <div className={classNames('row', { 'gx-4 gx-md-5': props.bigSize })}>
          <CategoryProgressbar
            name="Documentation"
            value={props.score.documentation}
            icon={CATEGORY_ICONS[ScoreType.Documentation]}
            bigSize={props.bigSize}
          />
          <CategoryProgressbar
            name="License"
            value={props.score.license}
            icon={CATEGORY_ICONS[ScoreType.License]}
            bigSize={props.bigSize}
          />
          <CategoryProgressbar
            name="Best Practices"
            value={props.score.best_practices}
            icon={CATEGORY_ICONS[ScoreType.BestPractices]}
            bigSize={props.bigSize}
          />
          <CategoryProgressbar
            name="Security"
            value={props.score.security}
            icon={CATEGORY_ICONS[ScoreType.Security]}
            bigSize={props.bigSize}
          />
          <CategoryProgressbar
            name="Legal"
            value={props.score.legal}
            icon={CATEGORY_ICONS[ScoreType.Legal]}
            bigSize={props.bigSize}
          />
        </div>
      </div>
    </div>
  );
};

export default CategoriesSummary;
