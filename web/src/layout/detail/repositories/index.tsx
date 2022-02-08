import { Fragment } from 'react';
import { GoLink } from 'react-icons/go';
import { VscGithub } from 'react-icons/vsc';
import { useLocation, useNavigate } from 'react-router-dom';

import { CATEGORY_ICONS } from '../../../data';
import { Report, Repository, ScoreType } from '../../../types';
import ExternalLink from '../../common/ExternalLink';
import RoundScore from '../../common/RoundScore';
import Row from '../report/Row';
import styles from './RepositoriesList.module.css';
import Summary from './Summary';

interface Props {
  repositories: Repository[];
  scrollIntoView: (id?: string) => void;
}

const RepositoriesList = (props: Props) => {
  const location = useLocation();
  const navigate = useNavigate();

  if (props.repositories.length === 0) return null;

  const getAnchorLink = (repo: Repository) => (
    <button
      onClick={() => {
        props.scrollIntoView(`#${repo.name}`);
        navigate(
          {
            pathname: location.pathname,
            hash: repo.name,
          },
          { state: location.state, replace: true }
        );
      }}
      className={`btn btn-link text-reset text-center lh-1 ${styles.headingLink}`}
      aria-label={`Go to ${repo.name}`}
    >
      <GoLink />
    </button>
  );

  return (
    <>
      <div className="my-3">
        <div className="text-uppercase h5 text-secondary fw-bold mb-3 mb-md-4">Repositories</div>
      </div>

      {/* Summary - only for more than 1 repository */}
      {props.repositories.length > 1 && <Summary repositories={props.repositories} />}

      {props.repositories.map((repo: Repository) => {
        return (
          <div key={`repo_${repo.repositoryId}`} className="mb-5 position-relative">
            <div>
              <div className={`position-absolute ${styles.headerAnchor}`} id={repo.name} />
            </div>
            <div className={`border px-4 py-3 py-md-4 ${styles.headerWrapper}`}>
              <div className="d-flex flex-row align-items-center">
                <div className="me-3">
                  <RoundScore score={repo.score.global} className={styles.global} />
                </div>
                <div className="mx-1 flex-grow-1 truncateWrapper position-relative">
                  <div className="d-none d-md-block">
                    <div className={`d-flex flex-row h4 fw-bold mb-2 ${styles.titleWrapper}`}>
                      <div className="text-truncate">{repo.name}</div>
                      {getAnchorLink(repo)}
                    </div>
                    <ExternalLink href={repo.url}>
                      <div className={`d-flex flex-row align-items-center ${styles.link}`}>
                        <VscGithub className="me-1" />
                        <div>{repo.url}</div>
                      </div>
                    </ExternalLink>
                  </div>
                  <div className="d-block d-md-none">
                    <div className="d-flex flex-row">
                      <ExternalLink href={repo.url} className="h5 fw-bold mb-0 text-truncate">
                        <div className="text-truncate">{repo.name}</div>
                      </ExternalLink>
                      {getAnchorLink(repo)}
                    </div>
                  </div>
                </div>
              </div>
            </div>
            <div>
              {repo.reports.map((report: Report) => {
                return (
                  <Fragment key={report.reportId}>
                    <Row
                      reportId={report.reportId}
                      name={ScoreType.Documentation}
                      label="Documentation"
                      data={report.data.documentation}
                      icon={CATEGORY_ICONS[ScoreType.Documentation]}
                      score={repo.score.documentation}
                    />
                    <Row
                      reportId={report.reportId}
                      name={ScoreType.License}
                      label="License"
                      data={report.data.license}
                      icon={CATEGORY_ICONS[ScoreType.License]}
                      score={repo.score.license}
                    />
                    <Row
                      reportId={report.reportId}
                      name={ScoreType.Quality}
                      label="Quality"
                      data={report.data.quality}
                      icon={CATEGORY_ICONS[ScoreType.Quality]}
                      score={repo.score.quality}
                    />
                    <Row
                      reportId={report.reportId}
                      name={ScoreType.Security}
                      label="Security"
                      data={report.data.security}
                      icon={CATEGORY_ICONS[ScoreType.Security]}
                      score={repo.score.security}
                    />
                  </Fragment>
                );
              })}
            </div>
          </div>
        );
      })}
    </>
  );
};

export default RepositoriesList;
