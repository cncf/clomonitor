import classNames from 'classnames';
import { RoundScore } from 'clo-ui/components/RoundScore';
import { isUndefined } from 'lodash';

import { SECTIONS } from '../../data';
import { ScoreType, SectionInfo } from '../../types';
import styles from './CategoriesSummary.module.css';
import CategoryProgressbar from './CategoryProgressbar';

interface Props {
  score: { [key in ScoreType]?: number | null };
  bigSize: boolean;
  repoName?: string;
  withLinks?: boolean;
  scrollIntoView?: (id?: string) => void;
}

const CategoriesSummary = (props: Props) => {
  const activeLink = !isUndefined(props.withLinks) && props.withLinks && props.repoName;

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
          <RoundScore score={props.score.global!} />
        </div>
      </div>

      <div
        className={classNames('flex-grow-1 w-100 position-relative', {
          'px-0 px-sm-3': props.bigSize,
        })}
      >
        <div className={classNames('row', { 'gx-4 gx-md-5': props.bigSize })}>
          {SECTIONS.map((section: SectionInfo) => (
            <CategoryProgressbar
              key={`category_${section.type}`}
              name={section.name}
              shortName={props.bigSize ? undefined : section.shortName}
              value={props.score[section.type]}
              icon={section.icon}
              bigSize={props.bigSize}
              linkTo={activeLink ? `${props.repoName}_${section.type}` : undefined}
              scrollIntoView={activeLink ? props.scrollIntoView : undefined}
            />
          ))}
        </div>
      </div>
    </div>
  );
};

export default CategoriesSummary;
