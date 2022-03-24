import classNames from 'classnames';
import { isNull, isUndefined } from 'lodash';
import moment from 'moment';
import { useCallback, useEffect, useState } from 'react';
import { GrPieChart } from 'react-icons/gr';
import { IoIosArrowBack } from 'react-icons/io';
import { useLocation, useNavigate, useParams } from 'react-router-dom';

import API from '../../api';
import useScrollRestorationFix from '../../hooks/useScrollRestorationFix';
import { ProjectDetail } from '../../types';
import updateMetaIndex from '../../utils/updateMetaIndex';
import CartegoryBadge from '../common/badges/CategoryBadge';
import MaturityBadge from '../common/badges/MaturityBadge';
import CategoriesSummary from '../common/CategoriesSummary';
import ExternalLink from '../common/ExternalLink';
import Image from '../common/Image';
import Loading from '../common/Loading';
import NoData from '../common/NoData';
import ProjectDropdown from '../common/ProjectDropdown';
import RoundScore from '../common/RoundScore';
import SubNavbar from '../navigation/SubNavbar';
import RepositorySection from '../search/RepositorySection';
import styles from './Detail.module.css';
import RepositoriesList from './repositories';

const Detail = () => {
  const navigate = useNavigate();
  const location = useLocation();
  const currentState = location.state as { currentSearch?: string };
  const { org, project } = useParams();
  const [detail, setDetail] = useState<ProjectDetail | null | undefined>();
  const [isLoadingProject, setIsLoadingProject] = useState<boolean>(false);

  useScrollRestorationFix();

  useEffect(() => {
    if (location.hash === '') {
      window.scrollTo(0, 0);
    } else {
      scrollIntoView();
    }
  }, [location]); /* eslint-disable-line react-hooks/exhaustive-deps */

  const scrollIntoView = useCallback(
    (id?: string) => {
      const elId = id || location.hash;
      if (isUndefined(elId) || elId === '') return;
      try {
        const element = document.getElementById(elId.replace('#', ''));
        if (element) {
          element.scrollIntoView({ block: 'start', inline: 'nearest', behavior: 'smooth' });
        }
      } finally {
        return;
      }
    },
    [location.hash]
  );

  useEffect(() => {
    async function fetchProjectDetail() {
      window.scrollTo(0, 0); // Go to top when a new project is fetched
      setIsLoadingProject(true);
      try {
        const projectDetail = await API.getProjectDetail(org!, project!);
        setDetail(projectDetail);
        updateMetaIndex(projectDetail.display_name || projectDetail.name, projectDetail.description);
        setIsLoadingProject(false);
      } catch (err: any) {
        setDetail(null);
        setIsLoadingProject(false);
      }
    }
    if (!isUndefined(org) && !isUndefined(project)) {
      fetchProjectDetail();
    }
  }, [org, project]);

  return (
    <>
      {currentState && currentState.currentSearch && (
        <SubNavbar>
          <button
            onClick={() => navigate(`/search${currentState.currentSearch}`)}
            className={`btn btn-link p-0 text-reset ${styles.backBtn}`}
            aria-label="Back to results"
          >
            <div className="d-flex flex-row align-items-center">
              <IoIosArrowBack className="me-2" />
              <div>Back to results</div>
            </div>
          </button>
        </SubNavbar>
      )}

      <main className="container-lg flex-grow-1 mb-0 mb-md-4">
        {isLoadingProject && <Loading transparentBg />}

        {!isUndefined(detail) && (
          <div
            className={classNames({
              'opacity-75': isLoadingProject,
            })}
          >
            {isNull(detail) ? (
              <div className="pt-5">
                <NoData>
                  <div className="mb-4 mb-lg-5 h2">Sorry, the project you requested was not found.</div>

                  <p className="h5 mb-0">The project you are looking for may have been deleted.</p>
                </NoData>
              </div>
            ) : (
              <>
                <div className="my-4 my-md-5">
                  <div className="border">
                    <div className="px-0 px-md-4 pt-0 pt-md-4">
                      <div className={`d-flex flex-row align-items-stretch px-3 py-2 p-md-0 ${styles.titleWrapper}`}>
                        <div
                          className={`d-flex align-items-center justify-content-center my-auto ${styles.imageWrapper}`}
                        >
                          <Image alt={`${detail.display_name || detail.name} logo`} url={detail.logo_url} />
                        </div>
                        <div className="d-flex flex-column justify-content-between ms-3 ms-sm-4 truncateWrapper">
                          <div className={`text-truncate fw-bold mb-0 ${styles.title}`}>
                            {detail.display_name || detail.name}
                          </div>

                          <div className="d-flex flex-row align-items-center my-2">
                            <MaturityBadge maturityLevel={detail.maturity_id} />
                            <CartegoryBadge categoryId={detail.category_id} className="d-none d-sm-block ms-2" />
                          </div>

                          <div className={`d-none d-sm-flex flex-row align-items-center ${styles.info}`}>
                            <RepositorySection repositories={detail.repositories} />

                            {detail.devstats_url && (
                              <>
                                <ExternalLink href={detail.devstats_url} className="ms-3">
                                  <div className={`d-flex flex-row align-items-center ${styles.link}`}>
                                    <GrPieChart className={`me-1 ${styles.statsIcon}`} />
                                    <div>DevStats</div>
                                  </div>
                                </ExternalLink>
                              </>
                            )}
                          </div>
                        </div>
                        <div className="d-none d-md-block ms-auto">
                          <div className="h-100 position-relative d-flex flex-column justify-content-between align-items-end">
                            {org && (
                              <ProjectDropdown
                                orgName={org}
                                projectName={detail.name}
                                projectDisplayName={detail.display_name}
                              />
                            )}
                          </div>
                        </div>
                        <div className="d-flex d-md-none align-items-center ms-auto">
                          <RoundScore score={detail.score.global} className={`ms-2 ${styles.global}`} />
                        </div>
                      </div>
                      <p className={`text-muted mt-3 mb-2 mt-md-4 mb-md-3 mx-3 mx-md-0 ${styles.description}`}>
                        {detail.description}
                      </p>
                      <div className={`text-muted fst-italic mx-3 mx-md-0 mb-2 mb-md-3 ${styles.updated}`}>
                        Updated {moment.unix(detail.updated_at).fromNow()}
                      </div>
                    </div>
                    <div className="pt-2">
                      <CategoriesSummary
                        score={detail.score}
                        repoName={detail.repositories.length === 1 ? detail.repositories[0].name : undefined}
                        scrollIntoView={scrollIntoView}
                        bigSize
                        withLinks
                      />
                    </div>
                  </div>
                </div>

                <RepositoriesList repositories={detail.repositories} scrollIntoView={scrollIntoView} />
              </>
            )}
          </div>
        )}
      </main>
    </>
  );
};

export default Detail;
