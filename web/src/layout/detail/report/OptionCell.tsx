import { DropdownOnHover } from 'clo-ui/components/DropdownOnHover';
import { ElementWithTooltip } from 'clo-ui/components/ElementWithTooltip';
import { ExternalLink } from 'clo-ui/components/ExternalLink';
import { FullScreenModal } from 'clo-ui/components/FullScreenModal';
import { isUndefined } from 'lodash';
import { useContext, useRef, useState } from 'react';
import { BsWindowPlus } from 'react-icons/bs';
import { FaRegCheckCircle, FaRegTimesCircle } from 'react-icons/fa';
import { FiExternalLink } from 'react-icons/fi';
import { MdRemoveCircleOutline } from 'react-icons/md';
import { RiErrorWarningLine } from 'react-icons/ri';
import ReactMarkdown from 'react-markdown';
import rehypeExternalLinks from 'rehype-external-links';

import { AppContext } from '../../../context/AppContextProvider';
import { REPORT_OPTIONS } from '../../../data';
import { ReportCheck, ReportOption, ReportOptionData } from '../../../types';
import styles from './OptionCell.module.css';

interface Props {
  label: ReportOption;
  check: ReportCheck;
  repoUrl?: string;
}

function getOptionInfo(key: ReportOption) {
  return REPORT_OPTIONS[key];
}

const OptionCell = (props: Props) => {
  const { ctx } = useContext(AppContext);
  const { effective } = ctx.prefs.theme;
  const iframe = useRef<HTMLIFrameElement>(null);
  const [openScoreModalStatus, setOpenScoreModalStatus] = useState<boolean>(false);
  const details = useRef<HTMLDivElement | null>(null);
  const errorIcon = <FaRegTimesCircle data-testid="error-icon" className={`text-danger ${styles.icon}`} />;
  const successIcon = <FaRegCheckCircle data-testid="success-icon" className={`text-success ${styles.icon}`} />;
  const exemptIcon = <MdRemoveCircleOutline data-testid="exempt-icon" className={`text-muted ${styles.exemptIcon}`} />;
  const failedIcon = <RiErrorWarningLine data-testid="failed-icon" className={styles.failedIcon} />;

  const opt: ReportOptionData = getOptionInfo(props.label);
  const scorecardBaseUrl = import.meta.env.DEV ? 'http://localhost:8000' : window.location.origin;

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const Heading = (props: any) => (
    <div className="fs-6 border-bottom border-1 pb-2 fw-bold w-100">{props.children}</div>
  );

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const Link = (data: any) => {
    return (
      <a href={data.href} target={data.target} rel="noopener noreferrer" className="text-decoration-underline">
        {data.children}
      </a>
    );
  };

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const Blockquote = (data: any) => {
    const content = data.children.find((el: { type: string }) => el.type === 'p');
    let el = data.children;
    if (content) {
      el = content.props.children;
    }
    return (
      <div className="pe-2">
        <div className={`w-100 overflow-auto ${styles.codeContent} ${styles.visibleScroll}`}>{el}</div>
      </div>
    );
  };

  const getCheckValue = (): string | JSX.Element => {
    let values;
    switch (props.label) {
      case ReportOption.SPDX:
        return <>{isUndefined(props.check.value) ? 'Not detected' : (props.check.value as string)}</>;

      case ReportOption.Analytics:
        values = isUndefined(props.check.value) ? [] : (props.check.value as string[]);
        return (
          <>
            {opt.name}
            {values.length > 0 && <span className="ms-2">({values.join(' · ')})</span>}
          </>
        );

      default:
        return opt.name;
    }
  };

  const scrollTop = () => {
    if (details && details.current) {
      details.current.scroll(0, 0);
    }
  };

  const getDetailsInfo = (): JSX.Element => {
    return (
      <>
        <div className="d-none d-lg-block">
          <DropdownOnHover
            width={700}
            dropdownClassName={styles.detailsDropdown}
            linkContent={
              <>
                {props.check.passed ? (
                  <div className="position-relative">
                    {successIcon}
                    <div className={`position-absolute bg-success rounded-circle ${styles.dot}`} />
                  </div>
                ) : (
                  <div className="position-relative">
                    {errorIcon}
                    <div className={`position-absolute bg-danger rounded-circle ${styles.dot}`} />
                  </div>
                )}
              </>
            }
            onClose={scrollTop}
            tooltipStyle
          >
            <div ref={details} className={`overflow-auto pb-1 ${styles.detailsWrapper} ${styles.visibleScroll}`}>
              <div className={styles.detailsContent}>
                <ReactMarkdown
                  rehypePlugins={[[rehypeExternalLinks, { rel: ['nofollow noreferrer noopener'], target: '_blank' }]]}
                  children={props.check.details!}
                  components={{
                    h1: Heading,
                    h2: Heading,
                    h3: Heading,
                    h4: Heading,
                    h5: Heading,
                    h6: Heading,
                    a: Link,
                    blockquote: Blockquote,
                  }}
                  skipHtml
                />
              </div>
            </div>
          </DropdownOnHover>
        </div>
        <span className="d-block d-lg-none">{props.check.passed ? successIcon : errorIcon}</span>
      </>
    );
  };

  const getScorecardInfo = (name: string): JSX.Element | null => {
    if (isUndefined(props.repoUrl)) return null;

    const url = new URL(props.repoUrl);
    const githubUrlParts = url.pathname.split('/');

    return (
      <>
        <button className={`btn btn-link text-reset p-0 ${styles.btn}`} onClick={() => setOpenScoreModalStatus(true)}>
          <div className="d-flex flex-row align-items-center w-100">
            <small className="fw-bold text-truncate">{name}</small>
            <BsWindowPlus className={`ms-2 ${styles.miniIcon}`} />
          </div>
        </button>
        <FullScreenModal
          excludedRefs={[iframe]}
          open={openScoreModalStatus}
          onClose={() => setOpenScoreModalStatus(false)}
        >
          <div className={`h-100 w-100 mx-auto ${styles.iframeWrapper}`}>
            <iframe
              ref={iframe}
              title="Scorecard details"
              src={`${scorecardBaseUrl}/scorecard?platform=${url.hostname}&org=${githubUrlParts[1]}&repo=${githubUrlParts[2]}&theme=${effective}&embed=true`}
              className={`w-100 ${styles.iframe}`}
            />
          </div>
        </FullScreenModal>
      </>
    );
  };

  const getIconCheck = (): JSX.Element => {
    if (!isUndefined(props.check.exempt) && props.check.exempt) {
      return (
        <>
          {!isUndefined(props.check.exemption_reason) && props.check.exemption_reason !== '' ? (
            <>
              <ElementWithTooltip
                element={
                  <div className="position-relative">
                    {exemptIcon}
                    <div className={`position-absolute bg-muted rounded-circle ${styles.dot}`} />
                  </div>
                }
                tooltipWidth={500}
                tooltipClassName={styles.reasonTooltipMessage}
                tooltipArrowClassName={styles.reasonTooltipArrow}
                tooltipMessage={
                  <div className="text-start p-2">
                    <div className="border-bottom border-1 pb-2 mb-3 fw-bold">
                      This repository is exempt from passing this check
                    </div>
                    <div className={`text-break ${styles.reason}`}>
                      <span className="fw-bold">Reason:</span> {props.check.exemption_reason}
                    </div>
                  </div>
                }
                alignmentTooltip="left"
                forceAlignment
                visibleTooltip
                active
              />
              <span className="d-block d-md-none">{exemptIcon}</span>
            </>
          ) : (
            <>{exemptIcon}</>
          )}
        </>
      );
    } else if (!isUndefined(props.check.failed) && props.check.failed) {
      return (
        <>
          {!isUndefined(props.check.fail_reason) && props.check.fail_reason !== '' ? (
            <>
              <ElementWithTooltip
                element={
                  <div className="position-relative">
                    {failedIcon}
                    <div className={`position-absolute bg-orange rounded-circle ${styles.dot}`} />
                  </div>
                }
                tooltipWidth={500}
                tooltipClassName={styles.reasonTooltipMessage}
                tooltipArrowClassName={styles.reasonTooltipArrow}
                tooltipMessage={
                  <div className="text-start p-2">
                    <div className="border-bottom border-1 pb-2 mb-3 fw-bold">
                      Something went wrong running this check
                    </div>
                    <div
                      ref={details}
                      className={`overflow-scroll ${styles.detailsWrapper} ${styles.visibleScroll} ${styles.reason} ${styles.failedReason}`}
                    >
                      <span className="fw-bold">Reason:</span> {props.check.fail_reason}
                    </div>
                  </div>
                }
                alignmentTooltip="left"
                forceAlignment
                visibleTooltip
                active
              />
              <span className="d-block d-md-none">{failedIcon}</span>
            </>
          ) : (
            <>{failedIcon}</>
          )}
        </>
      );
    } else {
      return (
        <>{props.check.details ? <> {getDetailsInfo()}</> : <> {props.check.passed ? successIcon : errorIcon}</>}</>
      );
    }
  };

  return (
    <tr>
      <td className={`text-center ${styles.iconCell}`}>{getIconCheck()}</td>
      <td className="pe-2 pe-md-4">
        <div className={`d-table w-100 ${styles.contentCell}`}>
          <div className="d-flex flex-row align-items-baseline align-items-md-center">
            <div className={`text-muted me-2 ${styles.iconCheck}`}>{opt.icon}</div>
            <div className="d-flex flex-column align-items-start flex-grow-1 truncateWrapper">
              <div data-testid="opt-name" className={`d-flex flex-row align-items-center w-100 ${styles.name}`}>
                {!isUndefined(props.check.url) ? (
                  <div>
                    {(() => {
                      switch (props.label) {
                        case ReportOption.OpenSSFScorecardBadge:
                          return <>{getScorecardInfo(opt.name)}</>;

                        default:
                          return (
                            <ExternalLink
                              href={props.check.url}
                              className="d-inline w-100"
                              label="Checks reference documentation"
                            >
                              <div className="d-flex flex-row align-items-center w-100">
                                <small className="fw-bold text-truncate">{getCheckValue()}</small>
                                <FiExternalLink className={`ms-2 ${styles.miniIcon}`} />
                              </div>
                            </ExternalLink>
                          );
                      }
                    })()}
                  </div>
                ) : (
                  <>
                    <small className="fw-bold text-truncate">{getCheckValue()}</small>
                  </>
                )}
              </div>
              <div className={`d-none d-md-flex flex-row text-muted w-100 ${styles.legend}`}>
                <div className="text-truncate">{opt.legend}</div>
                {opt.reference && (
                  <div className="d-none d-lg-flex text-nowrap">
                    <ExternalLink href={opt.reference} className="d-inline w-100 ms-1">
                      <div className="d-flex flex-row align-items-center w-100">
                        <div>[check docs</div>
                        <FiExternalLink className={`ms-1 ${styles.extraMiniIcon}`} />
                        <div>]</div>
                      </div>
                    </ExternalLink>
                  </div>
                )}
              </div>
            </div>
          </div>
        </div>
      </td>
    </tr>
  );
};

export default OptionCell;
