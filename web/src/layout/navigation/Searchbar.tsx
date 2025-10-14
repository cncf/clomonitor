import { SearchbarWithDropdown } from 'clo-ui/components/SearchbarWithDropdown';
import { Dispatch, SetStateAction, useContext } from 'react';
import { useNavigate, useSearchParams } from 'react-router-dom';

import API from '../../api';
import { AppContext } from '../../context/AppContextProvider';
import { DEFAULT_SORT_BY, DEFAULT_SORT_DIRECTION } from '../../data';
import prepareQueryString from '../../utils/prepareQueryString';

interface Props {
  setScrollPosition: Dispatch<SetStateAction<number | undefined>>;
  classNameWrapper?: string;
}

const Searchbar = (props: Props) => {
  const navigate = useNavigate();
  const [searchParams] = useSearchParams();
  const { ctx } = useContext(AppContext);
  const { effective } = ctx.prefs.theme;

  const search = (value: string) => {
    props.setScrollPosition(0);

    navigate({
      pathname: '/search',
      search: prepareQueryString({
        pageNumber: 1,
        text: value,
        filters: {},
      }),
    });
  };

  const cleanSearchValue = () => {
    props.setScrollPosition(0);
    navigate({
      pathname: '/search',
      search: prepareQueryString({
        pageNumber: 1,
        text: '',
        filters: {},
      }),
    });
  };

  const openProject = (foundation: string, projectName: string) => {
    navigate(`/projects/${foundation}/${projectName}`);
  };

  async function searchProjects(text: string) {
    try {
      const searchResults = await API.searchProjects({
        limit: 5,
        offset: 0,
        text: text,
        sort_by: DEFAULT_SORT_BY,
        sort_direction: DEFAULT_SORT_DIRECTION,
      });
      return searchResults;
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
    } catch (err: any) {
      return err;
    }
  }

  return (
    <div className={`position-relative ${props.classNameWrapper}`}>
      <SearchbarWithDropdown
        effective_theme={effective}
        searchProjects={(text: string) => searchProjects(text)}
        onSearch={(value: string) => search(value)}
        openProject={(foundation: string, projectName: string) => openProject(foundation, projectName)}
        onCleanSearchValue={cleanSearchValue}
        searchParams={searchParams}
      />
    </div>
  );
};

export default Searchbar;
