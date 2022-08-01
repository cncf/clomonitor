import { isNull } from 'lodash';
import { createContext, Dispatch, useContext, useEffect, useReducer, useState } from 'react';

import useSystemThemeMode from '../hooks/useSystemThemeMode';
import { Prefs, SortBy, SortDirection } from '../types';
import detectActiveThemeMode from '../utils/detectActiveThemeMode';
import getMetaTag from '../utils/getMetaTag';
import lsStorage from '../utils/localStoragePreferences';
import themeBuilder from '../utils/themeBuilder';

interface AppState {
  prefs: Prefs;
}

interface Props {
  children: JSX.Element;
}

const initialState: AppState = {
  prefs: lsStorage.getPrefs(),
};

type Action =
  | { type: 'updateTheme'; theme: string }
  | { type: 'updateEffectiveTheme'; theme: string }
  | { type: 'updateLimit'; limit: number }
  | { type: 'updateSort'; by: SortBy; direction: SortDirection };

export const AppContext = createContext<{
  ctx: AppState;
  dispatch: Dispatch<any>;
}>({
  ctx: initialState,
  dispatch: () => null,
});

export function updateTheme(theme: string) {
  return { type: 'updateTheme', theme };
}

export function updateEffectiveTheme(theme: string) {
  return { type: 'updateTheme', theme };
}

export function updateLimit(limit: number) {
  return { type: 'updateLimit', limit };
}

export function updateSort(by: SortBy, direction: SortDirection) {
  return { type: 'updateSort', by, direction };
}

export function updateActiveStyleSheet(current: string) {
  const secondary = getMetaTag('secondaryColor');
  document.getElementsByTagName('html')[0].setAttribute('data-theme', current);
  document
    .querySelector(`meta[name='theme-color']`)!
    .setAttribute('content', current === 'light' ? secondary : '#0f0e11');
}

export function appReducer(state: AppState, action: Action) {
  let prefs;
  switch (action.type) {
    case 'updateTheme':
      const effective = action.theme === 'automatic' ? detectActiveThemeMode() : action.theme;
      prefs = {
        ...state.prefs,
        theme: {
          configured: action.theme,
          effective: effective,
        },
      };

      lsStorage.setPrefs(prefs);
      updateActiveStyleSheet(effective);
      return {
        ...state,
        prefs: prefs,
      };

    case 'updateEffectiveTheme':
      prefs = {
        ...state.prefs,
        theme: {
          ...state.prefs.theme,
          effective: action.theme,
        },
      };
      lsStorage.setPrefs(prefs);
      updateActiveStyleSheet(action.theme);
      return {
        ...state,
        prefs: prefs,
      };

    case 'updateLimit':
      prefs = {
        ...state.prefs,
        search: {
          ...state.prefs.search,
          limit: action.limit,
        },
      };
      lsStorage.setPrefs(prefs);
      return {
        ...state,
        prefs: prefs,
      };

    case 'updateSort':
      prefs = {
        ...state.prefs,
        search: {
          ...state.prefs.search,
          sort: {
            by: action.by,
            direction: action.direction,
          },
        },
      };
      lsStorage.setPrefs(prefs);
      return {
        ...state,
        prefs: prefs,
      };

    default:
      return { ...state };
  }
}

function AppContextProvider(props: Props) {
  const activeProfilePrefs = lsStorage.getPrefs();
  const [ctx, dispatch] = useReducer(appReducer, {
    prefs: activeProfilePrefs,
  });
  const [activeInitialTheme, setActiveInitialTheme] = useState<string | null>(null);

  useEffect(() => {
    const theme =
      activeProfilePrefs.theme.configured === 'automatic'
        ? detectActiveThemeMode()
        : activeProfilePrefs.theme.configured;
    themeBuilder.init();
    updateActiveStyleSheet(theme);
    setActiveInitialTheme(theme);
  }, []); /* eslint-disable-line react-hooks/exhaustive-deps */

  useSystemThemeMode(ctx.prefs.theme.configured === 'automatic', dispatch);

  if (isNull(activeInitialTheme)) return null;

  return <AppContext.Provider value={{ ctx, dispatch }}>{props.children}</AppContext.Provider>;
}

function useAppContext() {
  return useContext(AppContext);
}

export { AppContextProvider, useAppContext };
