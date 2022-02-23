import moment from 'moment';
import { GrPieChart } from 'react-icons/gr';
import { useNavigate } from 'react-router-dom';

import { Project } from '../../types';
import CartegoryBadge from '../common/badges/CategoryBadge';
import MaturityBadge from '../common/badges/MaturityBadge';
import ExternalLink from '../common/ExternalLink';
import Image from '../common/Image';
import RoundScore from '../common/RoundScore';
import ScoreSummary from '../common/ScoreSummary';
import styles from './Card.module.css';
import RepositorySection from './RepositorySection';

interface Props {
  project: Project;
  currentQueryString: string;
  saveScrollPosition: () => void;
}

const Card = (props: Props) => {
  const navigate = useNavigate();

  return (
    <div className={`col-12 col-sm-6 col-md-12 col-lg-6 col-xxxl-4 ${styles.cardWrapper}`} role="listitem">
      <div
        className={`card rounded-0 p-0 p-md-3 p-lg-0 p-xl-3 h-100 mw-100 d-flex text-reset text-decoration-none ${styles.card} card`}
        role="button"
        onClick={() => {
          props.saveScrollPosition();
          navigate(`/projects/${props.project.organization.name}/${props.project.name}`, {
            state: { currentSearch: props.currentQueryString },
          });
        }}
      >
        <div className="d-flex flex-column flex-sm-row align-items-center">
          <div
            className={`d-none d-md-flex d-lg-none d-xl-flex align-items-center justify-content-center ${styles.imageWrapper}`}
          >
            <Image alt={`${props.project.display_name || props.project.name} logo`} url={props.project.logo_url} />
          </div>
          <div className="flex-grow-1 ms-0 ms-md-3 ms-lg-0 ms-xl-3 w-100 truncateWrapper">
            <div className={`p-2 p-md-3 p-lg-2 p-xl-3 ${styles.content}`}>
              <div className="d-flex flex-row align-items-center">
                <div
                  className={`d-flex d-md-none d-lg-flex d-xl-none align-items-center justify-content-center me-3 ${styles.miniImageWrapper}`}
                >
                  <Image
                    alt={`${props.project.display_name || props.project.name} logo`}
                    url={props.project.logo_url}
                  />
                </div>
                <div className="d-flex flex-column w-100 truncateWrapper">
                  <div className="d-flex flex-row justify-content-between align-items-end">
                    <span className={`text-truncate fw-bold mb-0 ${styles.title}`}>
                      {props.project.display_name || props.project.name}
                    </span>
                  </div>

                  <div className="d-flex flex-row align-items-center my-2">
                    <MaturityBadge maturityLevel={props.project.maturity_id} />
                    <CartegoryBadge
                      categoryId={props.project.category_id}
                      className="d-none d-md-block d-lg-none d-xxl-block ms-2"
                    />
                  </div>

                  <div className={`d-none d-md-flex d-lg-none d-xl-flex flex-row align-items-center ${styles.info}`}>
                    <RepositorySection repositories={props.project.repositories} />

                    {props.project.devstats_url && (
                      <>
                        <ExternalLink label="Dev stats link" href={props.project.devstats_url} className="ms-3">
                          <div className={`d-flex flex-row align-items-center ${styles.link}`}>
                            <GrPieChart className={`me-1 ${styles.statsIcon}`} />
                            <div>DevStats</div>
                          </div>
                        </ExternalLink>
                      </>
                    )}
                  </div>
                </div>

                <div className="d-flex d-md-none d-lg-flex d-xl-none">
                  <RoundScore score={props.project.score.global} className={`ms-2 ${styles.global}`} />
                </div>
              </div>
            </div>
          </div>
        </div>

        <p className={`text-muted mx-3 my-3 my-md-4 ${styles.description}`}>{props.project.description}</p>

        <div className="mt-auto">
          <ScoreSummary score={props.project.score} bigSize={false} />
        </div>
        <div className={`d-none d-md-block d-lg-none d-xl-block text-end text-muted fst-italic mt-2 ${styles.legend}`}>
          Updated {moment.unix(props.project.updated_at).fromNow()}
        </div>
      </div>
    </div>
  );
};

export default Card;
