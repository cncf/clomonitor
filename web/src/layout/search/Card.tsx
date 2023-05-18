import {
  Card as CardWrapper,
  ElementWithTooltip,
  ExternalLink,
  FoundationBadge,
  Image,
  MaturityBadge,
  RoundScore,
} from 'clo-ui';
import { isUndefined } from 'lodash';
import moment from 'moment';
import { useContext } from 'react';
import { FaChartBar } from 'react-icons/fa';
import { GoCalendar } from 'react-icons/go';
import { useNavigate } from 'react-router-dom';

import { AppContext } from '../../context/AppContextProvider';
import { Project } from '../../types';
import CategoriesSummary from '../common/CategoriesSummary';
import styles from './Card.module.css';
import RepositorySection from './RepositorySection';
import WebsiteSection from './WebsiteSection';

interface Props {
  project: Project;
  currentQueryString: string;
  saveScrollPosition: () => void;
}

const Card = (props: Props) => {
  const navigate = useNavigate();
  const { ctx } = useContext(AppContext);
  const { effective } = ctx.prefs.theme;

  return (
    <CardWrapper
      className={`p-md-3 p-lg-0 p-xl-3 ${styles.card} card`}
      wrapperClassName={`col-12 col-sm-6 col-md-12 col-lg-6 col-xxxl-4 ${styles.cardWrapper}`}
      onClick={() => {
        props.saveScrollPosition();
        navigate(`/projects/${props.project.foundation}/${props.project.name}`, {
          state: { currentSearch: props.currentQueryString },
        });
      }}
      hoverable
    >
      <>
        <div className="d-flex flex-column flex-sm-row align-items-center">
          <div
            className={`d-none d-md-flex d-lg-none d-xl-flex align-items-center justify-content-center ${styles.imageWrapper}`}
          >
            <Image
              alt={`${props.project.display_name || props.project.name} logo`}
              url={props.project.logo_url}
              dark_url={props.project.logo_dark_url}
              effective_theme={effective}
            />
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
                    dark_url={props.project.logo_dark_url}
                    effective_theme={effective}
                  />
                </div>
                <div className="d-flex flex-column w-100 truncateWrapper">
                  <div className="d-flex flex-row justify-content-between align-items-end">
                    <span className={`text-truncate fw-bold mb-0 ${styles.title}`}>
                      {props.project.display_name || props.project.name}
                    </span>
                  </div>

                  <div className="d-flex flex-row align-items-center my-2">
                    <FoundationBadge foundation={props.project.foundation} />
                    {props.project.maturity && (
                      <MaturityBadge maturityLevel={props.project.maturity} className="d-none d-md-flex ms-2" />
                    )}
                  </div>

                  <div
                    className={`d-none d-md-flex d-lg-none d-xl-flex flex-row mt-0 mt-md-1 mt-lg-0 mt-xl-1 align-items-center ${styles.info}`}
                  >
                    <RepositorySection repositories={props.project.repositories} onlyIcon />

                    <WebsiteSection repositories={props.project.repositories} onlyIcon />

                    {props.project.devstats_url && (
                      <>
                        <ExternalLink label="Dev stats link" href={props.project.devstats_url} className="ms-3">
                          <div className={`d-flex flex-row align-items-center text-muted ${styles.link}`}>
                            <FaChartBar className={styles.statsIcon} />
                          </div>
                        </ExternalLink>
                      </>
                    )}

                    {!isUndefined(props.project.accepted_at) && (
                      <ElementWithTooltip
                        element={
                          <div
                            className={`d-flex flex-row align-items-center ms-3 ${styles.subtitle} ${styles.wrapperCalendar}`}
                          >
                            <GoCalendar className={`me-1 text-muted ${styles.calendarIcon}`} />
                            <div>{moment.unix(props.project.accepted_at!).format('YYYY')}</div>
                          </div>
                        }
                        tooltipWidth={210}
                        tooltipClassName={styles.tooltipMessage}
                        tooltipMessage={
                          <div className="d-flex flex-column">
                            <div className="text-muted d-none d-lg-block">Accepted:</div>
                            <div className="lightText">
                              {moment.unix(props.project.accepted_at!).format('Do MMMM YYYY')}
                            </div>
                          </div>
                        }
                        visibleTooltip
                        active
                      />
                    )}
                  </div>
                </div>

                <div className="d-flex d-md-none d-lg-flex d-xl-none">
                  <RoundScore score={props.project.score.global!} className={`ms-2 ${styles.global}`} />
                </div>
              </div>
            </div>
          </div>
        </div>

        <p className={`text-muted mx-3 my-3 my-md-4 ${styles.description}`}>{props.project.description}</p>

        <div className="mt-auto">
          <CategoriesSummary score={props.project.score} bigSize={false} />
        </div>
        <div className={`d-none d-md-block d-lg-none d-xl-block text-end text-muted fst-italic mt-2 ${styles.legend}`}>
          Updated {moment.unix(props.project.updated_at).fromNow()}
        </div>
      </>
    </CardWrapper>
  );
};

export default Card;
