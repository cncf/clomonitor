import { FaCrown } from 'react-icons/fa';
import { VscGithub } from 'react-icons/vsc';
import { useLocation, useNavigate } from 'react-router-dom';

import { CATEGORY_ICONS } from '../../../data';
import { Repository, RepositoryKind, ScoreType } from '../../../types';
import ElementWithTooltip from '../../common/ElementWithTooltip';
import Badge from './Badge';
import styles from './Summary.module.css';

interface Props {
  repositories: Repository[];
}

const Summary = (props: Props) => {
  const navigate = useNavigate();
  const location = useLocation();

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
              <span className="d-inline-block d-md-none d-lg-inline-block ms-1 ms-xl-2">Global</span>
            </th>
            <th scope="col" className="d-none d-md-table-cell text-center text-nowrap">
              <small className={`position-relative ${styles.icon}`}>{CATEGORY_ICONS[ScoreType.Documentation]}</small>
              <span className="d-none d-lg-inline-block ms-1 ms-xl-2">Documentation</span>
            </th>
            <th scope="col" className="d-none d-md-table-cell text-center text-nowrap">
              <small className={`position-relative ${styles.icon}`}>{CATEGORY_ICONS[ScoreType.License]}</small>
              <span className="d-none d-lg-inline-block ms-1 ms-xl-2">License</span>
            </th>
            <th scope="col" className="d-none d-md-table-cell text-center text-nowrap">
              <small className={`position-relative ${styles.icon}`}>{CATEGORY_ICONS[ScoreType.BestPractices]}</small>
              <span className="d-none d-lg-inline-block ms-1 ms-xl-2">Best Practices</span>
            </th>
            <th scope="col" className="d-none d-md-table-cell text-center text-nowrap">
              <small className={`position-relative ${styles.icon}`}>{CATEGORY_ICONS[ScoreType.Security]}</small>
              <span className="d-none d-lg-inline-block ms-1 ms-xl-2">Security</span>
            </th>
            <th scope="col" className="d-none d-md-table-cell text-center text-nowrap">
              <small className={`position-relative ${styles.icon}`}>{CATEGORY_ICONS[ScoreType.Legal]}</small>
              <span className="d-none d-lg-inline-block ms-1 ms-xl-2">Legal</span>
            </th>
          </tr>
        </thead>
        <tbody>
          {props.repositories.map((repo: Repository) => {
            return (
              <tr key={`summary_${repo.repository_id}`}>
                <td className={`align-middle ${styles.repoCell} ${styles.darkBgCell}`}>
                  <div className="d-flex flex-row align-items-center pe-2">
                    <button
                      className={`btn btn-link text-dark text-truncate fw-bold px-2 ${styles.repoBtn}`}
                      onClick={() =>
                        navigate(
                          {
                            pathname: location.pathname,
                            hash: repo.name,
                          },
                          { state: location.state }
                        )
                      }
                      aria-label={`Go from summary to section: ${repo.name}`}
                    >
                      {repo.name}
                    </button>
                    {repo.kind === RepositoryKind.Primary && (
                      <>
                        <FaCrown className="d-block d-md-none text-warning" />
                        <ElementWithTooltip
                          className="lh-1"
                          element={<FaCrown className="text-warning" />}
                          tooltipWidth={210}
                          tooltipClassName={styles.tooltipMessage}
                          tooltipMessage={<div>Project's primary repository</div>}
                          alignmentTooltip="left"
                          forceAlignment
                          visibleTooltip
                          active
                        />
                      </>
                    )}
                  </div>
                </td>
                <td className="align-middle">
                  <Badge value={repo.score.global} />
                </td>
                <td className="d-none d-md-table-cell align-middle">
                  <Badge value={repo.score.documentation} />
                </td>
                <td className="d-none d-md-table-cell align-middle">
                  <Badge value={repo.score.license} />
                </td>
                <td className="d-none d-md-table-cell align-middle">
                  <Badge value={repo.score.best_practices} />
                </td>
                <td className="d-none d-md-table-cell align-middle">
                  <Badge value={repo.score.security} />
                </td>
                <td className="d-none d-md-table-cell align-middle">
                  <Badge value={repo.score.legal} />
                </td>
              </tr>
            );
          })}
        </tbody>
      </table>
    </div>
  );
};

export default Summary;
