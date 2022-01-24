import { Prefs } from '../types';

export interface PreferencesList {
  [key: string]: Prefs;
}

const LS_ITEM = 'remonitorPrefs';
export const DEFAULT_SEARCH_LIMIT = 20;
const DEFAULT_THEME = 'light';

const DEFAULT_PREFS: Prefs = {
  search: { limit: DEFAULT_SEARCH_LIMIT },
  theme: { effective: DEFAULT_THEME },
};

export class LocalStoragePreferences {
  private savedPreferences: PreferencesList = { guest: DEFAULT_PREFS };

  constructor() {
    try {
      const preferences = window.localStorage.getItem(LS_ITEM);
      if (preferences) {
        this.savedPreferences = JSON.parse(preferences);
      } else {
        this.setPrefs(DEFAULT_PREFS);
      }
    } catch {
      // Incognite mode
    }
  }

  public setPrefs(prefs: Prefs) {
    let preferences = { ...this.savedPreferences, guest: prefs };
    this.savedPreferences = preferences;

    try {
      window.localStorage.setItem(LS_ITEM, JSON.stringify(preferences));
    } catch {
      // Incognite mode
    }
  }

  public getPrefs(): Prefs {
    let prefs: Prefs = {
      ...DEFAULT_PREFS,
      ...this.savedPreferences.guest,
    };
    return prefs;
  }
}

const lsPreferences = new LocalStoragePreferences();
export default lsPreferences;
