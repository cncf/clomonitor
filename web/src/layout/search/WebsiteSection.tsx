import classNames from 'classnames';
import { isUndefined } from 'lodash';
import { useEffect, useState } from 'react';
import { IoGlobeOutline } from 'react-icons/io5';

import { BaseRepository, Repository } from '../../types';
import DropdownOnHover from '../common/DropdownOnHover';
import ExternalLink from '../common/ExternalLink';
import styles from './WebsiteSection.module.css';

interface Props {
  repositories: BaseRepository[] | Repository[];
  onlyIcon?: boolean;
}

const WebsiteSection = (props: Props) => {
  const [websites, setWebsites] = useState<string[]>([]);
  const isOnlyIcon = !isUndefined(props.onlyIcon) && props.onlyIcon;

  useEffect(() => {
    let urls: string[] = [];
    props.repositories.forEach((repo: BaseRepository | Repository) => {
      if (repo.hasOwnProperty('report')) {
        const currentRepo = repo as Repository;
        if (
          currentRepo.report &&
          currentRepo.report.data &&
          currentRepo.report.data.documentation &&
          currentRepo.report.data.documentation.website &&
          currentRepo.report.data.documentation.website.url
        ) {
          urls.push(currentRepo.report.data.documentation.website.url);
        }
      } else if (!isUndefined(repo.website_url)) {
        urls.push(repo.website_url);
      }
    });
    setWebsites(urls);
  }, [props.repositories]); /* eslint-disable-line react-hooks/exhaustive-deps */

  if (websites.length === 0) return null;

  return (
    <div className="ms-3">
      {websites.length === 1 ? (
        <ExternalLink label="Website link" href={websites[0]}>
          <div
            className={classNames('d-flex flex-row align-items-center', styles.link, {
              [`text-muted ${styles.onlyIcon}`]: isOnlyIcon,
            })}
          >
            <IoGlobeOutline className={styles.icon} />
            {!isOnlyIcon && <div className="ms-1">Website</div>}
          </div>
        </ExternalLink>
      ) : (
        <DropdownOnHover
          width={250}
          linkContent={
            <div
              className={classNames('d-flex flex-row align-items-center', styles.link, {
                [`text-muted ${styles.onlyIcon}`]: isOnlyIcon,
              })}
            >
              <IoGlobeOutline className={styles.icon} />
              {!isOnlyIcon && <div className="ms-1">Websites</div>}
            </div>
          }
        >
          <>
            {websites.map((url: string, index: number) => {
              return (
                <div key={`repo_${index}`} className={`d-flex flex-row align-items-center my-1 ${styles.link}`}>
                  <IoGlobeOutline className={`me-2 position-relative ${styles.miniIcon}`} />
                  <div className="truncateWrapper">
                    <ExternalLink
                      label="Website link"
                      href={url}
                      className={`d-block text-truncate text-dark ${styles.link}`}
                      visibleExternalIcon
                    >
                      <div className="text-truncate">{url}</div>
                    </ExternalLink>
                  </div>
                </div>
              );
            })}
          </>
        </DropdownOnHover>
      )}
    </div>
  );
};

export default WebsiteSection;
