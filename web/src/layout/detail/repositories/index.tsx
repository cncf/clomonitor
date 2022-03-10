import { isUndefined, orderBy } from 'lodash';
import { Fragment, useEffect, useRef, useState } from 'react';
import { FaCrown } from 'react-icons/fa';
import { GoLink } from 'react-icons/go';
import { VscGithub } from 'react-icons/vsc';
import { useLocation, useNavigate } from 'react-router-dom';

import { CATEGORY_ICONS } from '../../../data';
import { Report, Repository, RepositoryKind, ScoreType } from '../../../types';
import ElementWithTooltip from '../../common/ElementWithTooltip';
import ExternalLink from '../../common/ExternalLink';
import RoundScore from '../../common/RoundScore';
import Row from '../report/Row';
import styles from './RepositoriesList.module.css';
import Summary from './Summary';

interface Props {
  repositories: Repository[];
  scrollIntoView: (id?: string) => void;
}

// Sort by repo kind, score global and alphabetically
// IMPORTANT: primary repo is sorted first due to repo kind is sorted alphabetically,
// if we add a new kind we need to revisit this
const sortRepos = (repos: Repository[]): Repository[] => {
  return orderBy(repos, ['kind', 'score.global', 'name'], ['asc', 'desc', 'asc']);
};

const RepositoriesList = (props: Props) => {
  const { hash, state, pathname } = useLocation();
  const navigate = useNavigate();
  const ref = useRef<HTMLDivElement | null>(null);
  const [repositories, setRepositories] = useState<Repository[]>([]);

  const getAnchorLink = (anchorName: string, className?: string): JSX.Element => (
    <button
      onClick={() => {
        props.scrollIntoView(`#${anchorName}`);
        navigate(
          {
            pathname: pathname,
            hash: `${anchorName}`,
          },
          { state: state }
        );
      }}
      className={`btn btn-link text-reset text-center lh-1 ${styles.headingLink} ${className}`}
      aria-label={`Link to anchor ${anchorName}`}
    >
      <GoLink />
    </button>
  );

  useEffect(() => {
    setRepositories(sortRepos(props.repositories));
  }, [props.repositories]);

  useEffect(() => {
    let timer: NodeJS.Timer | undefined;

    const cleanInterval = () => {
      if (!isUndefined(timer)) {
        clearInterval(timer);
      }
    };

    if (hash === '') {
      window.scrollTo(0, 0);
    } else {
      // We need to check if element is in the DOM
      timer = setInterval(() => {
        if (ref && ref.current) {
          props.scrollIntoView();
          cleanInterval();
        }
      }, 50);
    }

    return () => {
      cleanInterval();
    };
  }, []); /* eslint-disable-line react-hooks/exhaustive-deps */

  if (repositories.length === 0) return null;

  return (
    <div ref={ref}>
      <div className="my-3">
        <div className="text-center text-md-start text-uppercase h5 text-secondary fw-bold mb-3 mb-md-4">
          Repositories
        </div>
      </div>

      {/* Summary - only when more than 1 repository */}
      {repositories.length > 1 && <Summary repositories={repositories} scrollIntoView={props.scrollIntoView} />}

      {repositories.map((repo: Repository) => {
        return (
          <div
            data-testid="repository-info"
            key={`repo_${repo.repository_id}`}
            className="mb-4 mb-md-5 position-relative"
          >
            <div id={repo.name} className={`position-absolute ${styles.headerAnchor}`} />

            <div className={`border px-3 py-2 px-md-4 py-md-4 ${styles.headerWrapper}`}>
              <div className="d-flex flex-row flex-md-row-reverse align-items-center">
                <div className="mx-0 mx-md-1 flex-grow-1 truncateWrapper position-relative">
                  <div className="d-none d-md-block">
                    <div className={`d-flex flex-row h4 fw-bold mb-2 ${styles.titleWrapper}`}>
                      <div className="text-truncate">{repo.name}</div>
                      {repo.kind === RepositoryKind.Primary && (
                        <ElementWithTooltip
                          className="lh-1 ms-3"
                          element={
                            <small>
                              <FaCrown className="text-warning" />
                            </small>
                          }
                          tooltipWidth={210}
                          tooltipClassName={styles.tooltipMessage}
                          tooltipMessage={<div>Project's primary repository</div>}
                          visibleTooltip
                          active
                        />
                      )}
                      {getAnchorLink(repo.name)}
                    </div>
                    <ExternalLink href={repo.url}>
                      <div className={`d-flex flex-row align-items-center ${styles.link}`}>
                        <VscGithub className="me-1" />
                        <div>{repo.url}</div>
                      </div>
                    </ExternalLink>
                  </div>
                  <div className="d-block d-md-none">
                    <div className="d-flex flex-row align-items-center">
                      <ExternalLink href={repo.url} className={`fw-bold text-truncate ${styles.repoName}`}>
                        <div className="text-truncate">{repo.name}</div>
                      </ExternalLink>
                      {repo.kind === RepositoryKind.Primary && (
                        <small>
                          <FaCrown data-testid="primary-icon" className="d-block d-md-none text-warning ms-2" />
                        </small>
                      )}
                      {getAnchorLink(repo.name)}
                    </div>
                  </div>
                </div>
                <div className="ms-3 ms-md-0 me-0 me-md-3">
                  <RoundScore score={repo.score.global} className={styles.global} />
                </div>
              </div>
            </div>
            <div>
              {repo.reports.map((report: Report) => {
                return (
                  <Fragment key={report.report_id}>
                    <Row
                      repoName={repo.name}
                      reportId={report.report_id}
                      name={ScoreType.Documentation}
                      label="Documentation"
                      data={report.data.documentation}
                      icon={CATEGORY_ICONS[ScoreType.Documentation]}
                      score={repo.score.documentation}
                      recommendedTemplates={
                        repo.kind === RepositoryKind.Primary
                          ? [
                              {
                                name: 'CONTRIBUTING.md',
                                url: 'https://github.com/cncf/project-template/blob/main/CONTRIBUTING.md',
                              },
                              {
                                name: 'GOVERNANCE.md',
                                url: 'https://github.com/cncf/project-template/blob/main/GOVERNANCE.md',
                              },
                            ]
                          : [
                              {
                                name: 'CONTRIBUTING.md',
                                url: 'https://github.com/cncf/project-template/blob/main/CONTRIBUTING.md',
                              },
                            ]
                      }
                      getAnchorLink={getAnchorLink}
                    />
                    <Row
                      repoName={repo.name}
                      reportId={report.report_id}
                      name={ScoreType.License}
                      label="License"
                      data={report.data.license}
                      icon={CATEGORY_ICONS[ScoreType.License]}
                      score={repo.score.license}
                      getAnchorLink={getAnchorLink}
                    />
                    <Row
                      repoName={repo.name}
                      reportId={report.report_id}
                      name={ScoreType.BestPractices}
                      label="Best Practices"
                      data={report.data.best_practices}
                      icon={CATEGORY_ICONS[ScoreType.BestPractices]}
                      score={repo.score.best_practices}
                      getAnchorLink={getAnchorLink}
                    />
                    <Row
                      repoName={repo.name}
                      reportId={report.report_id}
                      name={ScoreType.Security}
                      label="Security"
                      data={report.data.security}
                      icon={CATEGORY_ICONS[ScoreType.Security]}
                      score={repo.score.security}
                      recommendedTemplates={[
                        {
                          name: 'SECURITY.md',
                          url: 'https://github.com/cncf/tag-security/blob/main/project-resources/templates/SECURITY.md',
                        },
                      ]}
                      getAnchorLink={getAnchorLink}
                    />
                    <Row
                      repoName={repo.name}
                      reportId={report.report_id}
                      name={ScoreType.Legal}
                      label="Legal"
                      data={report.data.legal}
                      icon={CATEGORY_ICONS[ScoreType.Legal]}
                      score={repo.score.legal}
                      getAnchorLink={getAnchorLink}
                    />
                  </Fragment>
                );
              })}
            </div>
          </div>
        );
      })}
    </div>
  );
};

export default RepositoriesList;
