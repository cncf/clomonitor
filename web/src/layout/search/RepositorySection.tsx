import { VscGithub } from 'react-icons/vsc';

import { BaseRepository, Repository } from '../../types';
import DropdownOnHover from '../common/DropdownOnHover';
import ExternalLink from '../common/ExternalLink';
import styles from './RepositorySection.module.css';

interface Props {
  repositories: BaseRepository[] | Repository[];
}

const RepositorySection = (props: Props) => {
  return (
    <>
      {props.repositories.length === 1 ? (
        <ExternalLink href={props.repositories[0].url}>
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
            {props.repositories.map((repo: Repository | BaseRepository, index: number) => {
              return (
                <ExternalLink href={repo.url} key={`repo_${index}`} className="text-dark" visibleExternalIcon>
                  <div className={`d-flex flex-row align-items-center ${styles.link}`}>
                    <VscGithub className={`me-2 position-relative ${styles.miniIcon}`} />
                    <div className={`text-nowrap text-truncate ${styles.linkName}`}>{repo.name}</div>
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
