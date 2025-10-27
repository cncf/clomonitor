import classNames from 'classnames';
import { DateRangeOpts } from 'clo-ui/components/DateRangeFilter';
import { Loading } from 'clo-ui/components/Loading';
import { NoData } from 'clo-ui/components/NoData';
import { Pagination } from 'clo-ui/components/Pagination';
import { PaginationLimitOptions } from 'clo-ui/components/PaginationLimitOptions';
import { SampleQueries } from 'clo-ui/components/SampleQueries';
import { Sidebar } from 'clo-ui/components/Sidebar';
import { SortOptions } from 'clo-ui/components/SortOptions';
import { SubNavbar } from 'clo-ui/components/SubNavbar';
import { useScrollRestorationFix } from 'clo-ui/hooks/useScrollRestorationFix';
import { isEmpty, isUndefined } from 'lodash';
import { Dispatch, SetStateAction, useContext, useEffect, useLayoutEffect, useRef, useState } from 'react';
import { FaFilter } from 'react-icons/fa';
import { IoMdCloseCircleOutline } from 'react-icons/io';
import { useLocation, useNavigate, useSearchParams } from 'react-router-dom';

import API from '../../api';
import { AppContext, updateLimit, updateSort } from '../../context/AppContextProvider';
import { QUERIES, SORT_OPTIONS } from '../../data';
import { FilterKind, Project, SearchFiltersURL, SortBy, SortDirection, SortOption } from '../../types';
import buildSearchParams from '../../utils/buildSearchParams';
import prepareQueryString from '../../utils/prepareQueryString';
import scrollToPosition from '../../utils/scrollToPosition';
import Card from './Card';
import Filters from './filters';
import styles from './Search.module.css';
import SelectedFilters from './SelectedFilters';

interface FiltersProp {
  [key: string]: string[];
}

interface Props {
  scrollPosition?: number;
  setInvisibleFooter: Dispatch<SetStateAction<boolean>>;
  setScrollPosition: Dispatch<SetStateAction<number | undefined>>;
}

const Search = (props: Props) => {
  const navigate = useNavigate();
  const location = useLocation();
  const currentState = location.state as { resetScrollPosition?: boolean };
  const resetScrollPosition = currentState?.resetScrollPosition;
  const { ctx, dispatch } = useContext(AppContext);
  const { limit, sort } = ctx.prefs.search;
  const [searchParams] = useSearchParams();
  const hasRestoredRef = useRef(false);
  const [text, setText] = useState<string | undefined>();
  const [acceptedFrom, setAcceptedFrom] = useState<string | undefined>();
  const [acceptedTo, setAcceptedTo] = useState<string | undefined>();
  const [filters, setFilters] = useState<FiltersProp>({});
  const [pageNumber, setPageNumber] = useState<number>(1);
  const [total, setTotal] = useState<number>(0);
  const [projects, setProjects] = useState<Project[] | null | undefined>();
  const [isLoading, setIsLoading] = useState<boolean>(false);
  const [apiError, setApiError] = useState<string | null>(null);

  useScrollRestorationFix();

  useEffect(() => {
    hasRestoredRef.current = false;
  }, [props.scrollPosition, resetScrollPosition]);

  const saveScrollPosition = () => {
    props.setScrollPosition(window.scrollY);
  };

  const onResetFilters = (): void => {
    props.setScrollPosition(0);
    navigate({
      pathname: '/search',
      search: prepareQueryString({
        pageNumber: 1,
        text: text,
        filters: {},
      }),
    });
  };

  const calculateOffset = (pNumber: number): number => {
    return pNumber && limit ? (pNumber - 1) * limit : 0;
  };

  const getCurrentFilters = (): SearchFiltersURL => {
    return {
      pageNumber: pageNumber,
      text: text,
      accepted_from: acceptedFrom,
      accepted_to: acceptedTo,
      filters: filters,
    };
  };

  const onAcceptedDateRangeChange = (dates: DateRangeOpts) => {
    props.setScrollPosition(0);
    navigate({
      pathname: '/search',
      search: prepareQueryString({
        ...getCurrentFilters(),
        accepted_from: dates.from,
        accepted_to: dates.to,
        pageNumber: 1,
      }),
    });
  };

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const updateCurrentPage = (searchChanges: any) => {
    props.setScrollPosition(0);
    navigate({
      pathname: '/search',
      search: prepareQueryString({
        ...getCurrentFilters(),
        pageNumber: 1,
        ...searchChanges,
      }),
    });
  };

  const onPageNumberChange = (pageNumber: number): void => {
    updateCurrentPage({
      pageNumber: pageNumber,
    });
  };

  const onFiltersChange = (name: string, value: string, checked: boolean): void => {
    const currentFilters = filters || {};
    let additionalFilters = {};
    let newFilters = isUndefined(currentFilters[name]) ? [] : currentFilters[name].slice();
    if (checked) {
      newFilters.push(value);
    } else {
      newFilters = newFilters.filter((el) => el !== value);
    }

    // Remove selected maturity levels when selected foundations is different to only one
    if (name === FilterKind.Foundation && newFilters.length !== 1) {
      additionalFilters = { [FilterKind.Maturity]: [] };
    }

    updateCurrentPage({
      filters: { ...currentFilters, [name]: newFilters, ...additionalFilters },
    });
  };

  const onChecksChange = (updatedFilters: FiltersProp) => {
    updateCurrentPage({
      filters: { ...filters, ...updatedFilters },
    });
  };

  const onPaginationLimitChange = (newLimit: number): void => {
    props.setScrollPosition(0);
    navigate({
      pathname: '/search',
      search: prepareQueryString({
        ...getCurrentFilters(),
        pageNumber: 1,
      }),
    });
    dispatch(updateLimit(newLimit));
  };

  const onSortChange = (value: string): void => {
    const opts = value.split('_');
    props.setScrollPosition(0);
    // Load pageNumber is forced before update Sorting criteria
    navigate(
      {
        pathname: '/search',
        search: prepareQueryString({
          ...getCurrentFilters(),
          pageNumber: 1,
        }),
      },
      { replace: true }
    );
    dispatch(updateSort(opts[0] as SortBy, opts[1] as SortDirection));
  };

  useEffect(() => {
    const formattedParams = buildSearchParams(searchParams);
    setText(formattedParams.text);
    setAcceptedFrom(formattedParams.accepted_from);
    setAcceptedTo(formattedParams.accepted_to);
    setFilters(formattedParams.filters || {});

    setPageNumber(formattedParams.pageNumber);

    async function searchProjects() {
      setIsLoading(true);
      props.setInvisibleFooter(true);
      try {
        const newSearchResults = await API.searchProjects({
          text: formattedParams.text,
          accepted_from: formattedParams.accepted_from,
          accepted_to: formattedParams.accepted_to,
          sort_by: sort.by,
          sort_direction: sort.direction,
          filters: formattedParams.filters || {},
          offset: calculateOffset(formattedParams.pageNumber),
          limit: limit,
        });
        const newTotal = parseInt(newSearchResults['Pagination-Total-Count']);
        setTotal(newTotal);
        setProjects(newSearchResults.items);
      } catch {
        // TODO - error
        setApiError('An error occurred searching projects.');
      } finally {
        setIsLoading(false);
        props.setInvisibleFooter(false);
        if (resetScrollPosition) {
          props.setScrollPosition(0);
          scrollToPosition();
        } else if (isUndefined(props.scrollPosition) || props.scrollPosition === 0) {
          scrollToPosition();
        }
      }
    }
    searchProjects();
  }, [searchParams, limit, sort.by, sort.direction, resetScrollPosition]);

  useLayoutEffect(() => {
    if (
      isUndefined(projects) ||
      projects === null ||
      isUndefined(props.scrollPosition) ||
      props.scrollPosition === 0 ||
      resetScrollPosition ||
      hasRestoredRef.current
    ) {
      return;
    }

    scrollToPosition(props.scrollPosition as number);
    hasRestoredRef.current = true;
  }, [projects, props.scrollPosition, resetScrollPosition]);

  const visibleFiltersLabels = !isEmpty(filters) || !isUndefined(acceptedFrom) || !isUndefined(acceptedTo);

  return (
    <>
      <SubNavbar>
        <div className="d-flex flex-column w-100">
          <div className="d-flex flex-column flex-sm-row align-items-center justify-content-between flex-nowrap">
            <div className="d-flex flex-row flex-md-column align-items-center align-items-md-start w-100 text-truncate">
              <Sidebar
                label="Filters"
                className="d-inline-block d-md-none me-2"
                wrapperClassName="d-inline-block px-4"
                buttonType={classNames('btn-primary btn-sm rounded-circle position-relative', styles.btnMobileFilters, {
                  [styles.btnMobileFiltersActive]: visibleFiltersLabels,
                })}
                buttonIcon={<FaFilter />}
                closeButtonClassName={styles.closeSidebar}
                closeButton={
                  <>
                    {isLoading ? (
                      <>
                        <Loading spinnerClassName={styles.spinner} noWrapper smallSize />
                        <span className="ms-2">Searching...</span>
                      </>
                    ) : (
                      <>See {total} results</>
                    )}
                  </>
                }
                leftButton={
                  <>
                    {visibleFiltersLabels && (
                      <div className="d-flex align-items-center">
                        <IoMdCloseCircleOutline className={`text-dark ${styles.resetBtnDecorator}`} />
                        <button
                          className="btn btn-link btn-sm p-0 ps-1 text-dark"
                          onClick={onResetFilters}
                          aria-label="Reset filters"
                        >
                          Reset
                        </button>
                      </div>
                    )}
                  </>
                }
                header={<div className="h6 text-uppercase mb-0 flex-grow-1">Filters</div>}
              >
                <div role="menu">
                  <Filters
                    device="mobile"
                    acceptedFrom={acceptedFrom}
                    acceptedTo={acceptedTo}
                    activeFilters={filters}
                    onChange={onFiltersChange}
                    onChecksChange={onChecksChange}
                    onAcceptedDateRangeChange={onAcceptedDateRangeChange}
                    visibleTitle={false}
                  />
                </div>
              </Sidebar>
              <div className={`text-truncate fw-bold w-100 ${styles.searchResults}`} role="status">
                {total > 0 && (
                  <span className="pe-1">
                    {calculateOffset(pageNumber) + 1} - {total < limit * pageNumber ? total : limit * pageNumber}{' '}
                    <span className="ms-1">of</span>{' '}
                  </span>
                )}
                {total}
                <span className="ps-1"> results </span>
                {text && text !== '' && (
                  <span className="d-none d-sm-inline ps-1">
                    for "<span className="fw-bold">{text}</span>"
                  </span>
                )}
              </div>
            </div>
            <div className="d-flex flex-wrap flex-row justify-content-sm-end mt-3 mt-sm-0 ms-0 ms-md-3 w-100">
              <SortOptions
                options={SORT_OPTIONS as SortOption[]}
                by={sort.by}
                direction={sort.direction}
                width={180}
                className="me-2 me-md-4"
                onSortChange={onSortChange}
              />
              <PaginationLimitOptions limit={limit} onPaginationLimitChange={onPaginationLimitChange} />
            </div>
          </div>

          <SelectedFilters
            acceptedFrom={acceptedFrom}
            acceptedTo={acceptedTo}
            filters={filters}
            onChange={onFiltersChange}
            onAcceptedDateRangeChange={onAcceptedDateRangeChange}
          />
        </div>
      </SubNavbar>

      <main role="main" className="container-lg flex-grow-1 mb-4 mb-md-5">
        {isLoading && (
          <Loading
            className={visibleFiltersLabels ? styles.loadingWithFilters : styles.loading}
            position="fixed"
            transparentBg
          />
        )}
        <div
          className={classNames('h-100 position-relative d-flex flex-row align-items-start', {
            'opacity-75': isLoading,
          })}
        >
          <aside
            className={`d-none d-md-block position-relative p-3 rounded-0 border border-1 mb-3 mb-lg-4 ${styles.sidebar}`}
            aria-label="Filters"
          >
            <Filters
              device="desktop"
              acceptedFrom={acceptedFrom}
              acceptedTo={acceptedTo}
              activeFilters={filters}
              onChange={onFiltersChange}
              onChecksChange={onChecksChange}
              onAcceptedDateRangeChange={onAcceptedDateRangeChange}
              onResetFilters={onResetFilters}
              visibleTitle
            />
          </aside>
          <div className={`d-flex flex-column flex-grow-1 mt-2 mt-md-3 ${styles.contentWrapper}`}>
            {apiError && (
              <NoData className={styles.extraMargin}>
                <div className="mb-4 mb-lg-5 h2">{apiError}</div>
                <p className="h5 mb-0">Please try again later.</p>
              </NoData>
            )}

            {projects && (
              <>
                {isEmpty(projects) ? (
                  <NoData>
                    <div className="h4">
                      We're sorry!
                      <p className="h6 mb-0 mt-3 lh-base">
                        <span> We can't seem to find any projects that match your search </span>
                        {text && (
                          <span className="ps-1">
                            for "<span className="fw-bold">{text}</span>"
                          </span>
                        )}
                        {!isEmpty(filters) ? <span className="ps-1">with the selected filters</span> : <>.</>}
                      </p>
                      <p className="h6 mb-0 mt-5 lh-base">
                        You can{' '}
                        {!isEmpty(filters) ? (
                          <button
                            className="btn btn-link text-dark fw-bold py-0 pb-1 px-0"
                            onClick={onResetFilters}
                            aria-label="Reset filters"
                          >
                            <u>reset the filters</u>
                          </button>
                        ) : (
                          <button
                            className="btn btn-link text-dark fw-bold py-0 pb-1 px-0"
                            onClick={() => {
                              props.setScrollPosition(0);
                              navigate({
                                pathname: '/search',
                                search: prepareQueryString({
                                  pageNumber: 1,
                                  filters: {},
                                }),
                              });
                            }}
                            aria-label="Browse all packages"
                          >
                            <u>browse all projects</u>
                          </button>
                        )}
                        <> or try a new search.</>
                      </p>
                      <div className="h5 d-flex flex-row align-items-end justify-content-center flex-wrap">
                        <SampleQueries
                          className="bg-light text-dark border-secondary text-dark"
                          queries={QUERIES}
                          maxQueriesNumber={5}
                          prepareQueryString={prepareQueryString}
                        />
                      </div>
                    </div>
                  </NoData>
                ) : (
                  <div className={`row g-4 g-xxl-0 ${styles.list}`} role="list">
                    {projects.map((item: Project) => (
                      <Card
                        project={item}
                        key={`card_${item.name}`}
                        currentQueryString={prepareQueryString(getCurrentFilters())}
                        saveScrollPosition={saveScrollPosition}
                      />
                    ))}
                  </div>
                )}
              </>
            )}

            <div className="mt-auto mx-auto">
              <Pagination
                limit={limit}
                offset={0}
                total={total}
                active={pageNumber}
                className="mt-4 mt-md-5 mb-0 mb-md-2"
                onChange={onPageNumberChange}
              />
            </div>
          </div>
        </div>
      </main>
    </>
  );
};

export default Search;
