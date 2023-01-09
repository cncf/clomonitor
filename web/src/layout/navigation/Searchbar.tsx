import classNames from 'classnames';
import { isNull } from 'lodash';
import { ChangeEvent, Dispatch, KeyboardEvent, SetStateAction, useEffect, useRef, useState } from 'react';
import { FiSearch } from 'react-icons/fi';
import { IoCloseSharp } from 'react-icons/io5';
import { useNavigate, useSearchParams } from 'react-router-dom';

import API from '../../api';
import { DEFAULT_SORT_BY, DEFAULT_SORT_DIRECTION } from '../../data';
import useBreakpointDetect from '../../hooks/useBreakpointDetect';
import useOutsideClick from '../../hooks/useOutsideClick';
import { Project } from '../../types';
import prepareQueryString from '../../utils/prepareQueryString';
import FoundationBadge from '../common/badges/FoundationBadge';
import HoverableItem from '../common/HoverableItem';
import Image from '../common/Image';
import RoundScore from '../common/RoundScore';
import styles from './Searchbar.module.css';

interface Props {
  setScrollPosition: Dispatch<SetStateAction<number | undefined>>;
  classNameWrapper?: string;
}

const SEARCH_DELAY = 3 * 100; // 300ms
const MIN_CHARACTERS_SEARCH = 2;

const Searchbar = (props: Props) => {
  const navigate = useNavigate();
  const [searchParams] = useSearchParams();
  const inputEl = useRef<HTMLInputElement>(null);
  const dropdownRef = useRef(null);
  const [value, setValue] = useState<string>('');
  const [currentSearch, setCurrentSearch] = useState<string | null>(null);
  const [projects, setProjects] = useState<Project[] | null>(null);
  const [totalProjectsNumber, setTotalProjectsNumber] = useState<number | null>(null);
  const [visibleDropdown, setVisibleDropdown] = useState<boolean>(false);
  const [highlightedItem, setHighlightedItem] = useState<number | null>(null);
  const [dropdownTimeout, setDropdownTimeout] = useState<NodeJS.Timeout | null>(null);
  const point = useBreakpointDetect();

  useOutsideClick([dropdownRef], visibleDropdown, () => {
    cleanProjectsSearch();
  });

  const onKeyDown = (e: KeyboardEvent<HTMLInputElement>): void => {
    switch (e.key) {
      case 'Escape':
        cleanProjectsSearch();
        return;
      case 'ArrowDown':
        updateHighlightedItem('down');
        return;
      case 'ArrowUp':
        updateHighlightedItem('up');
        return;
      case 'Enter':
        e.preventDefault();
        if (!isNull(projects) && !isNull(highlightedItem)) {
          if (highlightedItem === projects.length) {
            search();
          } else {
            const selectedProject = projects[highlightedItem];
            if (selectedProject) {
              goToProject(selectedProject);
            }
          }
        } else {
          search();
        }
        return;
      default:
        return;
    }
  };

  const goToProject = (selectedProject: Project) => {
    forceBlur();
    setValue('');
    cleanProjectsSearch();
    navigate(`/projects/${selectedProject.foundation}/${selectedProject.name}`);
  };

  const forceBlur = (): void => {
    if (!isNull(inputEl) && !isNull(inputEl.current)) {
      inputEl.current.blur();
    }
  };

  const forceFocus = (): void => {
    if (!isNull(inputEl) && !isNull(inputEl.current)) {
      inputEl.current.focus();
    }
  };

  const search = () => {
    props.setScrollPosition(0);
    cleanTimeout();
    cleanProjectsSearch();

    navigate({
      pathname: '/search',
      search: prepareQueryString({
        pageNumber: 1,
        text: value,
        filters: {},
      }),
    });
    forceBlur();
  };

  const cleanTimeout = () => {
    if (!isNull(dropdownTimeout)) {
      clearTimeout(dropdownTimeout);
      setDropdownTimeout(null);
    }
  };

  const cleanSearchValue = () => {
    if (currentSearch === value) {
      props.setScrollPosition(0);
      navigate({
        pathname: '/search',
        search: prepareQueryString({
          pageNumber: 1,
          text: '',
          filters: {},
        }),
      });
    } else {
      setValue('');
    }
    forceFocus();
  };

  const cleanProjectsSearch = () => {
    setProjects(null);
    setTotalProjectsNumber(null);
    setVisibleDropdown(false);
    setHighlightedItem(null);
  };

  const updateHighlightedItem = (arrow: 'up' | 'down') => {
    if (!isNull(projects) && visibleDropdown) {
      if (!isNull(highlightedItem)) {
        let newIndex: number = arrow === 'up' ? highlightedItem - 1 : highlightedItem + 1;
        if (newIndex > projects.length) {
          newIndex = 0;
        }
        if (newIndex < 0) {
          newIndex = projects.length;
        }
        setHighlightedItem(newIndex);
      } else {
        if (projects && projects.length > 0) {
          const newIndex = arrow === 'up' ? projects.length : 0; // We don't subtract 1 because See all results (x) has to be count
          setHighlightedItem(newIndex);
        }
      }
    }
  };

  async function searchProjects() {
    try {
      const searchResults = await API.searchProjects({
        limit: 5,
        offset: 0,
        text: value,
        sort_by: DEFAULT_SORT_BY,
        sort_direction: DEFAULT_SORT_DIRECTION,
      });
      const total = parseInt(searchResults['Pagination-Total-Count']);
      if (total > 0) {
        const isInputFocused = inputEl.current === document.activeElement;
        // We have to be sure that input has focus to display results
        if (isInputFocused) {
          setProjects(searchResults.items);
          setTotalProjectsNumber(total);
          setVisibleDropdown(true);
        } else {
          cleanProjectsSearch();
        }
      } else {
        cleanProjectsSearch();
      }
    } catch (err: any) {
      cleanProjectsSearch();
    }
  }

  useEffect(() => {
    const text = searchParams.get('text');
    setValue(text || '');
    setCurrentSearch(text);
  }, [searchParams]);

  useEffect(() => {
    // Don't display search options for mobile devices
    if (point !== 'xs') {
      const isInputFocused = inputEl.current === document.activeElement;
      if (value.length >= MIN_CHARACTERS_SEARCH && isInputFocused) {
        cleanTimeout();
        setDropdownTimeout(
          setTimeout(() => {
            setHighlightedItem(null);
            searchProjects();
          }, SEARCH_DELAY)
        );
      } else {
        cleanProjectsSearch();
      }
    }

    return () => {
      if (!isNull(dropdownTimeout)) {
        clearTimeout(dropdownTimeout);
      }
    };
  }, [value]); /* eslint-disable-line react-hooks/exhaustive-deps */

  return (
    <div className={`position-relative ${props.classNameWrapper}`}>
      <div
        className={`d-flex align-items-center overflow-hidden searchBar lh-base bg-white mx-auto ${styles.searchBar} search`}
      >
        <input
          data-testid="search-bar"
          ref={inputEl}
          className={`flex-grow-1 ps-2 ps-md-3 border-0 shadow-none bg-transparent lh-base ${styles.input}`}
          type="text"
          value={value}
          autoComplete="off"
          autoCorrect="off"
          autoCapitalize="none"
          spellCheck="false"
          placeholder="Search projects"
          onKeyDown={onKeyDown}
          onChange={(e: ChangeEvent<HTMLInputElement>) => setValue(e.target.value)}
        />

        {value !== '' && (
          <>
            <button
              aria-label="Clear search"
              className={`btn btn-link lh-1 px-2 ${styles.btnIcon}`}
              onClick={cleanSearchValue}
            >
              <div className="text-muted lightIcon">
                <IoCloseSharp />
              </div>
            </button>
            <div className={`vr ${styles.vr}`} />
          </>
        )}

        <button aria-label="Search text" className={`btn btn-link lh-1 px-2 ${styles.btnIcon}`} onClick={search}>
          <div className={`${styles.iconWrapper} lightIcon`}>
            <FiSearch />
          </div>
        </button>
      </div>

      {visibleDropdown && !isNull(projects) && (
        <div
          ref={dropdownRef}
          className={`dropdown-menu dropdown-menu-left p-0 shadow-sm w-100 rounded-0 show noFocus ${styles.dropdown}`}
          role="listbox"
          id="search-list"
        >
          <HoverableItem onLeave={() => setHighlightedItem(null)}>
            <>
              {projects.map((project: Project, index: number) => {
                return (
                  <HoverableItem
                    key={`pkg_${project.id}`}
                    onHover={() => setHighlightedItem(index)}
                    onLeave={() => setHighlightedItem(null)}
                  >
                    <button
                      type="button"
                      className={classNames(
                        'btn btn-link w-100 border-bottom rounded-0 d-flex flex-row align-items-stretch text-decoration-none text-dark p-3',
                        styles.btnProject,
                        { [styles.activeDropdownItem]: index === highlightedItem }
                      )}
                      onClick={() => {
                        goToProject(project);
                      }}
                      aria-label={`Open package ${project.display_name || project.name} detail`}
                      role="option"
                      aria-selected={index === highlightedItem}
                      id={`sl-opt${index}`}
                    >
                      <div className="d-flex flex-row align-items-center w-100">
                        <div
                          className={`d-flex align-items-center justify-content-center me-2 ${styles.miniImageWrapper}`}
                        >
                          <Image alt={`${project.name}`} url={project.logo_url} dark_url={project.logo_dark_url} />
                        </div>
                        <div className="flex-grow-1 d-flex flex-column w-100 truncateWrapper">
                          <div className="d-flex flex-row justify-content-between align-items-end">
                            <span className={`text-truncate fw-bold mb-0 ${styles.title}`}>
                              {project.display_name || project.name}
                            </span>
                          </div>

                          <div className="d-flex flex-row align-items-center mt-1">
                            <FoundationBadge foundation={project.foundation} />
                          </div>
                        </div>

                        <div>
                          <RoundScore score={project.score.global!} className={`ms-2 ${styles.global}`} />
                        </div>
                      </div>
                    </button>
                  </HoverableItem>
                );
              })}

              <HoverableItem
                onHover={() => setHighlightedItem(projects.length)}
                onLeave={() => setHighlightedItem(null)}
              >
                <button
                  type="button"
                  className={classNames('btn btn-link w-100 text-dark p-2', styles.dropdownItem, {
                    [styles.activeDropdownItem]: projects.length === highlightedItem,
                  })}
                  onClick={search}
                  aria-label="See all results"
                  role="option"
                  aria-selected={projects.length === highlightedItem}
                  id={`sl-opt${projects.length}`}
                >
                  See all results ({totalProjectsNumber})
                </button>
              </HoverableItem>
            </>
          </HoverableItem>
        </div>
      )}
    </div>
  );
};

export default Searchbar;
