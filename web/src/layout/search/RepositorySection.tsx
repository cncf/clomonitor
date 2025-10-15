import classNames from 'classnames';
import { DropdownOnHover } from 'clo-ui/components/DropdownOnHover';
import { ExternalLink } from 'clo-ui/components/ExternalLink';
import { isUndefined } from 'lodash';
import { useEffect, useState } from 'react';
import { VscGithub } from 'react-icons/vsc';

import { BaseRepository, Repository } from '../../types';
import sortRepos from '../../utils/sortRepos';
import styles from './RepositorySection.module.css';

interface Props {
  repositories: BaseRepository[] | Repository[];
  onlyIcon?: boolean;
}

const RepositorySection = (props: Props) => {
  const [repositories, setRepositories] = useState<BaseRepository[] | Repository[]>([]);
  const isOnlyIcon = !isUndefined(props.onlyIcon) && props.onlyIcon;

  useEffect(() => {
    setRepositories(sortRepos(props.repositories as Repository[]));
  }, [props.repositories]);

  return (
    <>
      {repositories.length === 1 ? (
        <ExternalLink label="Repository link" href={repositories[0].url}>
          <div
            className={classNames('d-flex flex-row align-items-center', styles.link, {
              [`text-muted ${styles.onlyIcon}`]: isOnlyIcon,
            })}
          >
            <VscGithub className={styles.icon} />
            {!isOnlyIcon && <div className="ms-1">Repository</div>}
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
              <VscGithub className={styles.icon} />
              {!isOnlyIcon && <div className="ms-1">Repositories</div>}
            </div>
          }
        >
          <>
            {repositories.map((repo: Repository | BaseRepository, index: number) => {
              return (
                <div key={`repo_${index}`} className={`d-flex flex-row align-items-center my-1 ${styles.link}`}>
                  <VscGithub className={`me-2 position-relative ${styles.miniIcon}`} />
                  <div className="truncateWrapper">
                    <ExternalLink
                      label="Repository link"
                      href={repo.url}
                      className={`d-block text-truncate text-dark ${styles.link}`}
                      visibleExternalIcon
                    >
                      <div className="text-truncate">{repo.name}</div>
                    </ExternalLink>
                  </div>
                </div>
              );
            })}
          </>
        </DropdownOnHover>
      )}
    </>
  );
};

export default RepositorySection;
