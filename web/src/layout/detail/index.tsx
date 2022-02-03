import { isNull, isUndefined } from 'lodash';
import { useCallback, useEffect, useState } from 'react';
import { GrPieChart } from 'react-icons/gr';
import { IoIosArrowBack } from 'react-icons/io';
import { useLocation, useNavigate, useParams } from 'react-router-dom';

import API from '../../api';
import { ProjectDetail } from '../../types';
import CartegoryBadge from '../common/CategoryBadge';
import ExternalLink from '../common/ExternalLink';
import Image from '../common/Image';
import Loading from '../common/Loading';
import MaturityBadge from '../common/MaturityBadge';
import NoData from '../common/NoData';
import ProjectDropdown from '../common/ProjectDropdown';
import SubNavbar from '../common/SubNavbar';
import Summary from '../common/Summary';
import RepositorySection from '../search/RepositorySection';
import styles from './Detail.module.css';
import RepositoriesList from './repositories';

const Detail = () => {
  const navigate = useNavigate();
  const location = useLocation();
  const { projectId } = useParams();
  const [detail, setDetail] = useState<ProjectDetail | null | undefined>();
  const [isLoadingProject, setIsLoadingProject] = useState<boolean>(false);

  useEffect(() => {
    async function fetchProjectDetail() {
      setIsLoadingProject(true);
      try {
        setDetail(await API.getProjectDetail(projectId!));
        setIsLoadingProject(false);
      } catch (err: any) {
        setDetail(null);
        setIsLoadingProject(false);
      }
    }
    if (!isUndefined(projectId)) {
      fetchProjectDetail();
    }
  }, [projectId]);

  const scrollIntoView = useCallback(
    (id?: string) => {
      const elId = id || location.hash;
      if (isUndefined(elId) || elId === '') return;

      try {
        const element = document.querySelector(elId);
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
    scrollIntoView();
  }, [location.hash, detail]); /* eslint-disable-line react-hooks/exhaustive-deps */

  return (
    <>
      <SubNavbar>
        <button onClick={() => navigate(-1)} className="btn btn-link p-0 text-reset">
          <div className="d-flex flex-row align-items-center">
            <IoIosArrowBack className="me-2" />
            <div>Back to results</div>
          </div>
        </button>
      </SubNavbar>

      <main className="container-lg flex-grow-1 mb-4">
        {isLoadingProject && <Loading />}

        {!isUndefined(detail) && (
          <>
            {isNull(detail) ? (
              <div className="pt-5">
                <NoData>
                  <div className="mb-4 mb-lg-5 h2">Sorry, the project you requested was not found.</div>

                  <p className="h5 mb-0">The project you are looking for may have been deleted...</p>
                </NoData>
              </div>
            ) : (
              <>
                <div className="my-5">
                  <div className="border">
                    <div className="px-4 pt-4">
                      <div className="d-flex flex-row align-items-stretch">
                        <div className={`d-flex align-items-center justify-content-center ${styles.imageWrapper}`}>
                          <Image alt={`${detail.name}`} url={detail.logoUrl} />
                        </div>
                        <div className="d-flex flex-column justify-content-between ms-2 ms-sm-4 truncateWrapper">
                          <div className={`text-truncate fw-bold mb-0 ${styles.title}`}>
                            {detail.displayName || detail.name}
                          </div>

                          <div className="d-flex flex-row align-items-center my-2">
                            <MaturityBadge maturityLevel={detail.maturityId} />
                            <CartegoryBadge categoryId={detail.categoryId} className="d-none d-sm-block ms-2" />
                          </div>

                          <div className={`d-none d-sm-flex flex-row align-items-center ${styles.info}`}>
                            <RepositorySection repositories={detail.repositories} />

                            {detail.devstatsUrl && (
                              <>
                                <ExternalLink href={detail.devstatsUrl} className="ms-3">
                                  <div className={`d-flex flex-row align-items-center ${styles.link}`}>
                                    <GrPieChart className={`me-1 ${styles.statsIcon}`} />
                                    <div>DevStats</div>
                                  </div>
                                </ExternalLink>
                              </>
                            )}
                          </div>
                        </div>
                        <div className="ms-auto">
                          <div className="h-100 position-relative d-flex flex-column justify-content-between align-items-end">
                            <ProjectDropdown />
                          </div>
                        </div>
                      </div>
                      <p className="text-muted my-4">{detail.description}</p>
                    </div>
                    <div className="pt-2">
                      <Summary score={detail.score} bigSize />
                    </div>
                  </div>
                </div>

                <RepositoriesList repositories={detail.repositories} scrollIntoView={scrollIntoView} />
              </>
            )}
          </>
        )}
      </main>
    </>
  );
};

export default Detail;
