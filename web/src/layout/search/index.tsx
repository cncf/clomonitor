import { isEmpty, isUndefined } from 'lodash';
import { useContext, useEffect, useState } from 'react';
import { FaFilter } from 'react-icons/fa';
import { IoMdCloseCircleOutline } from 'react-icons/io';
import { useNavigate, useSearchParams } from 'react-router-dom';

import API from '../../api';
import { AppContext, updateLimit, updateSort } from '../../context/AppContextProvider';
import { Project, SearchFiltersURL, SortBy, SortDirection } from '../../types';
import buildSearchParams from '../../utils/buildSearchParams';
import { prepareQueryString } from '../../utils/prepareQueryString';
import Loading from '../common/Loading';
import NoData from '../common/NoData';
import Pagination from '../common/Pagination';
import PaginationLimit from '../common/PaginationLimit';
import SampleQueries from '../common/SampleQueries';
import Sidebar from '../common/Sidebar';
import SortOptions from '../common/SortOptions';
import SubNavbar from '../common/SubNavbar';
import Card from './Card';
import Filters from './filters';
import styles from './Search.module.css';
import SelectedFilters from './SelectedFilters';

interface FiltersProp {
  [key: string]: (string | number)[];
}

const prepareFilters = (filters: FiltersProp): FiltersProp => {
  let f: FiltersProp = { ...filters };
  Object.keys(filters).forEach((key: string) => {
    if (['maturity', 'category'].includes(key)) {
      f[key] = (f[key] as string[]).map((v: string) => parseInt(v));
    }
  });
  return f;
};

const Search = () => {
  const navigate = useNavigate();
  const { ctx, dispatch } = useContext(AppContext);
  const { limit, sort } = ctx.prefs.search;
  const [searchParams] = useSearchParams();
  const [text, setText] = useState<string | undefined>();
  const [filters, setFilters] = useState<FiltersProp>({});
  const [pageNumber, setPageNumber] = useState<number>(1);
  const [total, setTotal] = useState<number>(0);
  const [projects, setProjects] = useState<Project[] | null | undefined>();
  const [isLoading, setIsLoading] = useState<boolean>(false);

  const onResetFilters = (): void => {
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
      filters: filters,
    };
  };

  const updateCurrentPage = (searchChanges: any) => {
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

  const updateWindowScrollPosition = (newPosition: number) => {
    window.scrollTo(0, newPosition);
  };

  const onFiltersChange = (name: string, value: string, checked: boolean): void => {
    const currentFilters = filters || {};
    let newFilters = isUndefined(currentFilters[name]) ? [] : currentFilters[name].slice();
    if (checked) {
      newFilters.push(value);
    } else {
      newFilters = newFilters.filter((el) => el !== value);
    }

    updateCurrentPage({
      filters: { ...currentFilters, [name]: newFilters },
    });
  };

  const onPaginationLimitChange = (newLimit: number): void => {
    navigate({
      pathname: '/search',
      search: prepareQueryString({
        ...getCurrentFilters(),
        pageNumber: 1,
      }),
    });
    dispatch(updateLimit(newLimit));
  };

  const onSortChange = (by: SortBy, direction: SortDirection): void => {
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
    dispatch(updateSort(by, direction));
  };

  useEffect(() => {
    const formattedParams = buildSearchParams(searchParams);
    setText(formattedParams.text);
    setFilters(formattedParams.filters || {});
    setPageNumber(formattedParams.pageNumber);

    async function searchProjects() {
      setIsLoading(true);
      try {
        const data = {
          text: formattedParams.text,
          sortBy: sort.by,
          sortDirection: sort.direction,
          filters: prepareFilters(formattedParams.filters || {}),
          offset: calculateOffset(formattedParams.pageNumber),
          limit: limit,
        };
        const newSearchResults = await API.searchProjects(data);
        setTotal(parseInt(newSearchResults.paginationTotalCount));
        setProjects(newSearchResults.items);
        updateWindowScrollPosition(0);
      } catch {
        // TODO - error
      } finally {
        setIsLoading(false);
      }
    }
    searchProjects();
  }, [searchParams, limit, sort.by, sort.direction]); /* eslint-disable-line react-hooks/exhaustive-deps */

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
                buttonType={`btn-primary btn-sm rounded-circle position-relative ${styles.btnMobileFilters}`}
                buttonIcon={<FaFilter />}
                closeButton={<>See {total} results</>}
                leftButton={
                  <>
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
                  </>
                }
                header={<div className="h6 text-uppercase mb-0 flex-grow-1">Filters</div>}
              >
                <div role="menu">
                  <Filters device="mobile" activeFilters={filters} onChange={onFiltersChange} visibleTitle={false} />
                </div>
              </Sidebar>
              <div className={`text-truncate fw-bold ${styles.searchResults}`} role="status">
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
              <SortOptions by={sort.by} direction={sort.direction} onSortChange={onSortChange} />
              <PaginationLimit onPaginationLimitChange={onPaginationLimitChange} />
            </div>
          </div>

          <SelectedFilters filters={filters} onChange={onFiltersChange} />
        </div>
      </SubNavbar>

      <main role="main" className="container-lg flex-grow-1 mb-4">
        {isLoading && <Loading position="fixed" />}
        <div className="h-100 position-relative d-flex flex-row align-items-start">
          <aside
            className={`d-none d-md-block position-relative p-3 rounded-0 border mb-3 mb-lg-4 ${styles.sidebar}`}
            aria-label="Filters"
          >
            <Filters
              device="desktop"
              activeFilters={filters}
              onChange={onFiltersChange}
              onResetFilters={onResetFilters}
              visibleTitle
            />
          </aside>
          <div className={`d-flex flex-column flex-grow-1 mt-3 ${styles.contentWrapper}`}>
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
                        <SampleQueries className="bg-light text-dark border-secondary text-dark" />
                      </div>
                    </div>
                  </NoData>
                ) : (
                  <div className={`row g-4 g-xxl-0 ${styles.list}`} role="list">
                    {projects.map((item: Project) => (
                      <Card project={item} key={`card_${item.name}`} />
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
                className="mt-5"
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
