import { VscGithub } from 'react-icons/vsc';
import { useLocation, useNavigate } from 'react-router-dom';

import { CATEGORY_ICONS } from '../../../data';
import { Repository, ScoreType } from '../../../types';
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
    <div className="pt-2 mb-5 table-responsive">
      <table className={`table table-bordered mb-0 ${styles.table}`}>
        <thead>
          <tr>
            <th scope="col" className="text-center text-nowrap">
              <small className={`me-2 position-relative ${styles.icon}`}>
                <VscGithub />
              </small>
              <span>Repository</span>
            </th>
            <th scope="col" className="text-center text-nowrap">
              <small className={`me-2 position-relative ${styles.icon}`}>{CATEGORY_ICONS[ScoreType.Global]}</small>
              <span>Global</span>
            </th>
            <th scope="col" className="text-center text-nowrap">
              <small className={`me-2 position-relative ${styles.icon}`}>
                {CATEGORY_ICONS[ScoreType.Documentation]}
              </small>
              <span>Documentation</span>
            </th>
            <th scope="col" className="text-center text-nowrap">
              <small className={`me-2 position-relative ${styles.icon}`}>{CATEGORY_ICONS[ScoreType.License]}</small>
              <span>License</span>
            </th>
            <th scope="col" className="text-center text-nowrap">
              <small className={`me-2 position-relative ${styles.icon}`}>{CATEGORY_ICONS[ScoreType.Quality]}</small>
              <span>Code quality</span>
            </th>
            <th scope="col" className="text-center text-nowrap">
              <small className={`me-2 position-relative ${styles.icon}`}>{CATEGORY_ICONS[ScoreType.Security]}</small>
              <span>Security</span>
            </th>
          </tr>
        </thead>
        <tbody>
          {props.repositories.map((repo: Repository) => {
            return (
              <tr key={`summary_${repo.repositoryId}`}>
                <td className={`align-middle ${styles.darkBgCell}`}>
                  <button
                    className="btn btn-link text-dark text-truncate fw-bold"
                    onClick={() =>
                      navigate({
                        pathname: location.pathname,
                        hash: repo.name,
                      })
                    }
                  >
                    {repo.name}
                  </button>
                </td>
                <td className="align-middle">
                  <Badge value={repo.score.global} />
                </td>
                <td className="align-middle">
                  <Badge value={repo.score.documentation} />
                </td>
                <td className="align-middle">
                  <Badge value={repo.score.license} />
                </td>
                <td className="align-middle">
                  <Badge value={repo.score.quality} />
                </td>
                <td className="align-middle">
                  <Badge value={repo.score.security} />
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
