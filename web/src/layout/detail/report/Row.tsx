import { ElementWithTooltip } from 'clo-ui/components/ElementWithTooltip';
import { ExternalLink } from 'clo-ui/components/ExternalLink';
import { getCategoryColor } from 'clo-ui/utils/getCategoryColor';
import { roundScoreValue } from 'clo-ui/utils/roundScoreValue';
import { isNil, isUndefined } from 'lodash';
import { Fragment, useEffect, useState } from 'react';
import { FiExternalLink } from 'react-icons/fi';
import { IoInformationCircleOutline } from 'react-icons/io5';

import { RecommendedTemplate, ReportCheck, ReportOption, ScoreType } from '../../../types';
import sortChecks from '../../../utils/sortChecks';
import OptionCell from './OptionCell';
import styles from './Row.module.css';
import Title from './Title';

interface OptData {
  [key: string]: ReportCheck | undefined;
}

interface Props {
  repoName: string;
  reportId: string;
  name: ScoreType;
  label: string;
  icon: JSX.Element;
  data: OptData;
  score?: number | null;
  advisory?: boolean;
  referenceUrl?: string;
  recommendedTemplates?: RecommendedTemplate[];
  getAnchorLink: (anchorName: string, className?: string) => JSX.Element;
  repoUrl?: string;
}

const Row = (props: Props) => {
  const color = getCategoryColor(props.score ?? undefined);
  const [options, setOptions] = useState<ReportOption[]>([]);
  const tmplsNumber = props.recommendedTemplates ? props.recommendedTemplates.length : 0;
  const scoreValue = props.score ?? 0;
  const clampedScore = Math.max(0, Math.min(100, scoreValue));
  const barWidth = clampedScore === 0 ? 1 : clampedScore;

  useEffect(() => {
    setOptions(sortChecks(props.data));
  }, [props.data]);

  if (options.length === 0 || isNil(props.score)) return null;

  const advisoryIcon = props.advisory ? (
    <ElementWithTooltip
      element={
        <span
          data-testid="advisory-badge"
          className={`d-inline-flex align-items-center text-muted ms-2 ${styles.advisoryIcon}`}
          role="img"
          aria-label="Advisory section: it does not affect the global score"
        >
          <IoInformationCircleOutline aria-hidden="true" />
        </span>
      }
      tooltipWidth={260}
      tooltipClassName={styles.tooltipMessage}
      tooltipArrowClassName={styles.tooltipArrow}
      tooltipMessage={
        <div className="text-start p-2">This section has its own score but does not affect the global score.</div>
      }
      visibleTooltip
      active
    />
  ) : undefined;

  return (
    <div className={`p-3 p-md-4 border border-1 mb-2 ${styles.reportContent}`}>
      <div className="mx-0 mx-md-1">
        <div className="d-flex flex-row position-relative">
          <div id={`${props.repoName}_${props.name}`} className={`position-absolute ${styles.headerAnchor}`} />
          <Title
            title={props.label}
            icon={props.icon}
            className={styles.titleWrapper}
            extra={advisoryIcon}
            anchor={props.getAnchorLink(`${props.repoName}_${props.name}`, styles.headingLink)}
          />
        </div>
        <div className="d-flex flex-row mt-2 mb-4 align-items-center">
          <div className={`flex-grow-1 ${styles.progressbarWrapper}`}>
            <div className={`progress rounded-0 ${styles.progress}`}>
              <div
                className={`progress-bar ${styles.progressbar}`}
                role="progressbar"
                style={{ width: `${barWidth}%`, backgroundColor: `var(--clo-${color})` }}
                aria-valuenow={clampedScore}
                aria-valuemin={0}
                aria-valuemax={100}
                aria-label={`${props.label} score for ${props.repoName}`}
              />
            </div>
          </div>
          <div className={`ps-3 lh-1 ${styles.scoreWrapper}`}>
            <small className="fw-bold">{roundScoreValue(props.score)}%</small>
          </div>
        </div>
        <div>
          <table className={`table align-middle w-100 border ${styles.table}`}>
            <tbody>
              {options.map((opt: string) => {
                if (isUndefined(props.data[opt])) return null;
                return (
                  <OptionCell
                    key={`${props.reportId}_${props.label}_${opt}_cell`}
                    label={opt as ReportOption}
                    check={props.data[opt]!}
                    repoUrl={props.repoUrl}
                  />
                );
              })}
            </tbody>
          </table>
        </div>

        <ul className={`d-none d-md-block mb-0 ${styles.linksList}`}>
          {!isUndefined(props.referenceUrl) && (
            <li className="pt-2">
              <ExternalLink
                href={props.referenceUrl}
                label={`Checks reference documentation for ${props.label} category`}
                className="d-inline-block"
              >
                <div className="d-flex flex-row align-items-center">
                  <div>Checks reference documentation</div>
                  <FiExternalLink className={`ms-1 ms-md-2 position-relative ${styles.extIcon}`} />
                </div>
              </ExternalLink>
            </li>
          )}

          {!isUndefined(props.recommendedTemplates) && props.recommendedTemplates.length > 0 && (
            <li data-testid="recommended-templates" className="pt-1">
              <div className="d-flex flex-row align-items-center">
                <div>
                  Recommended templates:{' '}
                  {props.recommendedTemplates.map((tmpl: RecommendedTemplate, index: number) => {
                    return (
                      <Fragment key={`${props.label}_tmpl_${index}`}>
                        <ExternalLink
                          href={tmpl.url}
                          className="d-inline-block"
                          label={`Recommended template: ${tmpl.name}`}
                        >
                          <div className="d-flex flex-row align-items-center">
                            <code className="text-muted fw-bold">{tmpl.name}</code>
                            <FiExternalLink className={`ms-1 ms-md-2 position-relative ${styles.extIcon}`} />
                          </div>
                        </ExternalLink>
                        {(() => {
                          switch (index) {
                            case tmplsNumber - 1:
                              return <>.</>;

                            case tmplsNumber - 2:
                              return <> and </>;

                            default:
                              return <>, </>;
                          }
                        })()}
                      </Fragment>
                    );
                  })}
                </div>
              </div>
            </li>
          )}
        </ul>
      </div>
    </div>
  );
};

export default Row;
