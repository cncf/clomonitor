import './styles/default.scss';

import { useState } from 'react';
import { BrowserRouter, Navigate, Route, Routes } from 'react-router-dom';

import { AppContextProvider } from './context/AppContextProvider';
import Layout from './layout';
import Detail from './layout/detail';
import NotFound from './layout/notFound';
import Search from './layout/search';

function App() {
  const [scrollPosition, setScrollPosition] = useState<undefined | number>(undefined);

  return (
    <AppContextProvider>
      <BrowserRouter>
        <Routes>
          <Route path="/" element={<Layout setScrollPosition={setScrollPosition} />}>
            <Route path="/" element={<Navigate to="/search?page=1" replace />} />
            <Route
              path="search"
              element={<Search scrollPosition={scrollPosition} setScrollPosition={setScrollPosition} />}
            />
            <Route path="projects/:org/:project" element={<Detail />} />
            <Route path="*" element={<NotFound />} />
          </Route>
        </Routes>
      </BrowserRouter>
    </AppContextProvider>
  );
}

export default App;
