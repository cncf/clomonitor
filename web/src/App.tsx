import './styles/default.scss';

import { useState } from 'react';
import { BrowserRouter as Router, Navigate, Route, Routes, useParams } from 'react-router-dom';

import { AppContextProvider } from './context/AppContextProvider';
import Layout from './layout';
import Detail from './layout/detail';
import NotFound from './layout/notFound';
import Search from './layout/search';
import StatsView from './layout/stats';

// Old path without :foundation is redirected to CNCF foundation by default
const ProjectsRedirection = () => {
  const { org, project } = useParams();
  return <Navigate to={`/projects/cncf/${org}/${project}`} replace />;
};

function App() {
  const [scrollPosition, setScrollPosition] = useState<undefined | number>(undefined);

  return (
    <AppContextProvider>
      <Router>
        <Routes>
          <Route path="/" element={<Layout setScrollPosition={setScrollPosition} />}>
            <Route path="/" element={<Navigate to="/search?page=1" replace />} />
            <Route
              path="search"
              element={<Search scrollPosition={scrollPosition} setScrollPosition={setScrollPosition} />}
            />
            <Route path="projects/:org/:project" element={<ProjectsRedirection />} />
            <Route path="projects/:foundation/:org/:project" element={<Detail />} />
            <Route path="stats" element={<StatsView />} />
            <Route path="*" element={<NotFound />} />
          </Route>
        </Routes>
      </Router>
    </AppContextProvider>
  );
}

export default App;
