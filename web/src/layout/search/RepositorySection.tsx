import { useEffect, useState } from 'react';
import { VscGithub } from 'react-icons/vsc';

import { BaseRepository, Repository } from '../../types';
import sortRepos from '../../utils/sortRepos';
import DropdownOnHover from '../common/DropdownOnHover';
import ExternalLink from '../common/ExternalLink';
import styles from './RepositorySection.module.css';

interface Props {
  repositories: BaseRepository[] | Repository[];
}

const RepositorySection = (props: Props) => {
  const [repositories, setRepositories] = useState<BaseRepository[] | Repository[]>([]);

  useEffect(() => {
    setRepositories(sortRepos(props.repositories as Repository[]));
  }, [props.repositories]); /* eslint-disable-line react-hooks/exhaustive-deps */

  return (
    <>
      {repositories.length === 1 ? (
        <ExternalLink label="Repository link" href={repositories[0].url}>
          <div className={`d-flex flex-row align-items-center ${styles.link}`}>
            <VscGithub className={`me-1 ${styles.icon}`} />
            <div>Repository</div>
          </div>
        </ExternalLink>
      ) : (
        <DropdownOnHover
          width={250}
          linkContent={
            <div className={`d-flex flex-row align-items-center ${styles.link}`}>
              <VscGithub className={`me-1 ${styles.icon}`} />
              <div>Repositories</div>
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
