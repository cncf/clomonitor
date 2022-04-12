import { isUndefined } from 'lodash';
import { VscGithub } from 'react-icons/vsc';
import { useLocation, useNavigate } from 'react-router-dom';

import { CATEGORY_ICONS } from '../../../data';
import { Repository, ScoreType } from '../../../types';
import CheckSetBadge from '../../common/badges/CheckSetBadge';
import BadgeCell from './BadgeCell';
import styles from './Summary.module.css';

interface Props {
  repositories: Repository[];
  scrollIntoView: (id?: string) => void;
}

const Summary = (props: Props) => {
  const navigate = useNavigate();
  const location = useLocation();

  const goToAnchor = (hash: string) => {
    props.scrollIntoView(`#${hash}`);
    navigate(
      {
        pathname: location.pathname,
        hash: hash,
      },
      { state: location.state }
    );
  };

  if (props.repositories.length === 0) return null;

  return (
    <div className="pt-2 mb-4 mb-md-5">
      <table data-testid="repositories-summary" className={`table table-bordered mb-0 w-100 ${styles.table}`}>
        <thead>
          <tr>
            <th scope="col" className="text-center text-nowrap">
              <small className={`me-2 position-relative ${styles.icon}`}>
                <VscGithub />
              </small>
              <span>Repository</span>
            </th>
            <th scope="col" className="text-center text-nowrap">
              <small className={`position-relative ${styles.icon}`}>{CATEGORY_ICONS[ScoreType.Global]}</small>
              <span className="d-inline-block d-md-none d-xl-inline-block ms-1 ms-xl-2">Global</span>
            </th>
            <th scope="col" className="d-none d-md-table-cell text-center text-nowrap">
              <small className={`position-relative ${styles.icon}`}>{CATEGORY_ICONS[ScoreType.Documentation]}</small>
              <span className="d-none d-xl-inline-block ms-1 ms-xl-2">Documentation</span>
            </th>
            <th scope="col" className="d-none d-md-table-cell text-center text-nowrap">
              <small className={`position-relative ${styles.icon}`}>{CATEGORY_ICONS[ScoreType.License]}</small>
              <span className="d-none d-xl-inline-block ms-1 ms-xl-2">License</span>
            </th>
            <th scope="col" className="d-none d-md-table-cell text-center text-nowrap">
              <small className={`position-relative ${styles.icon}`}>{CATEGORY_ICONS[ScoreType.BestPractices]}</small>
              <span className="d-none d-xl-inline-block ms-1 ms-xl-2">Best Practices</span>
            </th>
            <th scope="col" className="d-none d-md-table-cell text-center text-nowrap">
              <small className={`position-relative ${styles.icon}`}>{CATEGORY_ICONS[ScoreType.Security]}</small>
              <span className="d-none d-xl-inline-block ms-1 ms-xl-2">Security</span>
            </th>
            <th scope="col" className="d-none d-md-table-cell text-center text-nowrap">
              <small className={`position-relative ${styles.icon}`}>{CATEGORY_ICONS[ScoreType.Legal]}</small>
              <span className="d-none d-xl-inline-block ms-1 ms-xl-2">Legal</span>
            </th>
          </tr>
        </thead>
        <tbody>
          {props.repositories.map((repo: Repository) => {
            if (isUndefined(repo.report)) return null;
            return (
              <tr key={`summary_${repo.repository_id}`}>
                <td className={`align-middle ${styles.repoCell} ${styles.darkBgCell}`}>
                  <div className="d-flex flex-row align-items-center">
                    <button
                      className={`btn btn-link text-dark text-truncate fw-bold px-2 py-0 py-xl-1 ${styles.repoBtn}`}
                      onClick={() => goToAnchor(repo.name)}
                      aria-label={`Go from summary to section: ${repo.name}`}
                    >
                      {repo.name}
                    </button>
                    <CheckSetBadge checkSets={repo.check_sets} className="d-none d-xl-inline-flex" />
                  </div>
                </td>

                <BadgeCell
                  value={!isUndefined(repo.score) ? repo.score.global : undefined}
                  cellClassName="align-middle"
                  onClick={() => goToAnchor(repo.name)}
                />

                <BadgeCell
                  value={!isUndefined(repo.score) ? repo.score.documentation : undefined}
                  onClick={() => goToAnchor(`${repo.name}_${ScoreType.Documentation}`)}
                />

                <BadgeCell
                  value={!isUndefined(repo.score) ? repo.score.license : undefined}
                  onClick={() => goToAnchor(`${repo.name}_${ScoreType.License}`)}
                />

                <BadgeCell
                  value={!isUndefined(repo.score) ? repo.score.best_practices : undefined}
                  onClick={() => goToAnchor(`${repo.name}_${ScoreType.BestPractices}`)}
                />

                <BadgeCell
                  value={!isUndefined(repo.score) ? repo.score.security : undefined}
                  onClick={() => goToAnchor(`${repo.name}_${ScoreType.Security}`)}
                />

                <BadgeCell
                  value={!isUndefined(repo.score) ? repo.score.legal : undefined}
                  onClick={() => goToAnchor(`${repo.name}_${ScoreType.Legal}`)}
                />
              </tr>
            );
          })}
        </tbody>
      </table>
    </div>
  );
};

export default Summary;
