import { sortBy } from 'lodash';
import { useEffect, useState } from 'react';
import { FaCrown } from 'react-icons/fa';
import { VscGithub } from 'react-icons/vsc';

import { BaseRepository, Repository, RepositoryKind } from '../../types';
import DropdownOnHover from '../common/DropdownOnHover';
import ExternalLink from '../common/ExternalLink';
import styles from './RepositorySection.module.css';

interface Props {
  repositories: BaseRepository[] | Repository[];
}

const sortRepos = (repos: BaseRepository[] | Repository[]): BaseRepository[] | Repository[] => {
  return sortBy(repos, 'kind');
};

const RepositorySection = (props: Props) => {
  const [repositories, setRepositories] = useState<BaseRepository[] | Repository[]>([]);

  useEffect(() => {
    setRepositories(sortRepos(props.repositories));
  }, [props.repositories]);

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
                <ExternalLink
                  label="Repository link"
                  href={repo.url}
                  key={`repo_${index}`}
                  className="text-dark"
                  visibleExternalIcon
                >
                  <div className={`d-flex flex-row align-items-center ${styles.link}`}>
                    <VscGithub className={`me-2 position-relative ${styles.miniIcon}`} />
                    <div className={`text-nowrap text-truncate ${styles.linkName}`}>{repo.name}</div>
                    {repo.kind === RepositoryKind.Primary && <FaCrown className="ms-2 text-warning" />}
                  </div>
                </ExternalLink>
              );
            })}
          </>
        </DropdownOnHover>
      )}
    </>
  );
};

export default RepositorySection;
