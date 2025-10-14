import { useSystemThemeMode } from 'clo-ui/hooks/useSystemThemeMode';
import { detectActiveThemeMode } from 'clo-ui/utils/detectActiveThemeMode';
import { getMetaTag } from 'clo-ui/utils/getMetaTag';
import { isNull } from 'lodash';
import { createContext, Dispatch, useContext, useEffect, useReducer, useState } from 'react';

import { Prefs, SortBy, SortDirection } from '../types';
import lsStorage from '../utils/localStoragePreferences';

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
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
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
  const themeColor = current === 'light' ? (secondary as string) : '#0f0e11';
  document.querySelector(`meta[name='theme-color']`)!.setAttribute('content', themeColor);
}

export function appReducer(state: AppState, action: Action) {
  let prefs;
  let effective;
  switch (action.type) {
    case 'updateTheme':
      effective = action.theme === 'automatic' ? detectActiveThemeMode() : action.theme;
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
        : activeProfilePrefs.theme.configured || activeProfilePrefs.theme.effective; // Use effective theme if configured is undefined
    updateActiveStyleSheet(theme);
    setActiveInitialTheme(theme);
  }, []);

  useSystemThemeMode(ctx.prefs.theme.configured === 'automatic', dispatch);

  if (isNull(activeInitialTheme)) return null;

  return <AppContext.Provider value={{ ctx, dispatch }}>{props.children}</AppContext.Provider>;
}

function useAppContext() {
  return useContext(AppContext);
}

export { AppContextProvider, useAppContext };
