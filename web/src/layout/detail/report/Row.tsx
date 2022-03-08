import { isUndefined } from 'lodash';
import React, { Fragment, useEffect, useState } from 'react';
import { FiExternalLink } from 'react-icons/fi';
import { IoHelpBuoySharp } from 'react-icons/io5';

import { RecommendedTemplate, ReportOption, ScoreType } from '../../../types';
import getCategoryColor from '../../../utils/getCategoryColor';
import ExternalLink from '../../common/ExternalLink';
import OptionCell from './OptionCell';
import styles from './Row.module.css';
import Title from './Title';

interface OptData {
  [key: string]: string | boolean;
}

interface Props {
  reportId: string;
  name: ScoreType;
  label: string;
  icon: JSX.Element;
  data: OptData;
  score: number;
  recommendedTemplates?: RecommendedTemplate[];
}

const sortOptions = (opts?: OptData): ReportOption[] => {
  if (isUndefined(opts)) return [];
  let optNames: ReportOption[] = [];
  Object.keys(opts).forEach((opt: string) => {
    // we check that opt belongs to ReportOption enum
    if (Object.values(ReportOption).includes(opt as ReportOption)) {
      optNames.push(opt as ReportOption);
    }
  });

  const sortedNames = optNames.sort((a, b) => {
    // spdxId is always first item in its category
    if (a === ReportOption.SPDX || b === ReportOption.SPDX) return -1;
    const nameA = a.toLowerCase();
    const nameB = b.toLowerCase();
    if (nameA < nameB) return -1;
    if (nameA > nameB) return 1;
    return 0;
  });

  return sortedNames;
};

const Row = (props: Props) => {
  const color = getCategoryColor(props.score);
  const [options, setOptions] = useState<ReportOption[]>([]);
  const tmplsNumber = props.recommendedTemplates ? props.recommendedTemplates.length : 0;

  useEffect(() => {
    setOptions(sortOptions(props.data));
  }, [props.data]);

  if (options.length === 0) return null;

  return (
    <div className="p-3 p-md-4 border border-top-0">
      <div className="mx-0 mx-md-1">
        <Title title={props.label} icon={props.icon} />
        <div className="d-flex flex-row mt-2 mb-4 align-items-center">
          <div className={`flex-grow-1 ${styles.progressbarWrapper}`}>
            <div className={`progress rounded-0 ${styles.progress}`}>
              <div
                className="progress-bar progress-bar-striped"
                role="progressbar"
                style={{ width: `${props.score || 1}%`, backgroundColor: `var(--rm-${color})` }}
              />
            </div>
          </div>
          <div className={`ps-3 lh-1 ${styles.scoreWrapper}`}>
            <small className="fw-bold">{props.score}%</small>
          </div>
        </div>
        <div>
          <table className={`table align-middle w-100 border ${styles.table}`}>
            <tbody>
              {options.map((opt: string) => {
                return (
                  <OptionCell
                    key={`${props.reportId}_${props.label}_${opt}_cell`}
                    label={opt as ReportOption}
                    value={props.data[opt]}
                  />
                );
              })}
            </tbody>
          </table>
        </div>

        {!isUndefined(props.recommendedTemplates) && props.recommendedTemplates.length > 0 && (
          <div data-testid="recommended-templates">
            <div className="d-flex flex-row align-items-center pt-2">
              <IoHelpBuoySharp className="me-2" />
              <div>
                CNCF recommended templates:{' '}
                {props.recommendedTemplates.map((tmpl: RecommendedTemplate, index: number) => {
                  return (
                    <Fragment key={`${props.label}_tmpl_${index}`}>
                      <ExternalLink href={tmpl.url} className="d-inline-block">
                        <div className="d-flex flex-row align-items-center">
                          <code className="text-muted fw-bold">{tmpl.name}</code>
                          <FiExternalLink className={`ms-2 position-relative ${styles.extIcon}`} />
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
          </div>
        )}
      </div>
    </div>
  );
};

export default Row;
