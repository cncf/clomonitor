import classNames from 'classnames';

import { CATEGORY_ICONS } from '../../data';
import { Score, ScoreType } from '../../types';
import Category from './Category';
import RoundScore from './RoundScore';
import styles from './Summary.module.css';

interface Props {
  score: Score;
  bigSize: boolean;
}

const SummaryCard = (props: Props) => {
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
        <RoundScore score={props.score.global} className="me-0 me-sm-4 mb-2 mb-sm-0" />
      </div>

      <div
        className={classNames('flex-grow-1 w-100 position-relative', styles.categories, {
          'px-0 px-sm-3': props.bigSize,
        })}
      >
        <div className={classNames('row', { 'gx-4 gx-md-5': props.bigSize })}>
          <Category
            name="Documentation"
            shortName="Docs"
            value={props.score.documentation}
            icon={CATEGORY_ICONS[ScoreType.Documentation]}
            bigSize={props.bigSize}
          />
          <Category
            name="License"
            value={props.score.license}
            icon={CATEGORY_ICONS[ScoreType.License]}
            bigSize={props.bigSize}
          />
        </div>
        <div className={classNames('row mt-2', { 'mt-md-3 gx-4 gx-md-5': props.bigSize })}>
          <Category
            name="Best Practices"
            value={props.score.bestPractices}
            icon={CATEGORY_ICONS[ScoreType.BestPractices]}
            bigSize={props.bigSize}
          />
          <Category
            name="Security"
            value={props.score.security}
            icon={CATEGORY_ICONS[ScoreType.Security]}
            bigSize={props.bigSize}
          />
        </div>
      </div>
    </div>
  );
};

export default SummaryCard;
