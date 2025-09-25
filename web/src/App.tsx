import 'clo-ui/dist/styles/xxxl_grid.scss';

import { AlertController } from 'clo-ui';
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
  const { project } = useParams();
  return <Navigate to={`/projects/cncf/${project}`} replace />;
};

function App() {
  const [scrollPosition, setScrollPosition] = useState<undefined | number>(undefined);
  const [invisibleFooter, setInvisibleFooter] = useState<boolean>(false);

  return (
    <AppContextProvider>
      <Router future={{ v7_startTransition: true }}>
        <AlertController />
        <Routes>
          <Route path="/" element={<Layout invisibleFooter={invisibleFooter} setScrollPosition={setScrollPosition} />}>
            <Route path="/" element={<Navigate to="/search?page=1" replace />} />
            <Route
              path="search"
              element={
                <Search
                  scrollPosition={scrollPosition}
                  setScrollPosition={setScrollPosition}
                  setInvisibleFooter={setInvisibleFooter}
                />
              }
            />
            <Route path="projects/:project" element={<ProjectsRedirection />} />
            <Route path="projects/:foundation/:project" element={<Detail setInvisibleFooter={setInvisibleFooter} />} />
            <Route path="stats" element={<StatsView />} />
            <Route path="*" element={<NotFound />} />
          </Route>
        </Routes>
      </Router>
    </AppContextProvider>
  );
}

export default App;
