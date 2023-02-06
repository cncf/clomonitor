import { detectActiveThemeMode } from 'clo-ui';
import { isEmpty, isUndefined, sortBy } from 'lodash';

import { DEFAULT_SORT_BY, DEFAULT_SORT_DIRECTION } from '../data';
import { Prefs } from '../types';

export interface PreferencesList {
  [key: string]: Prefs;
}

const LS_ITEM = 'clomonitorPrefs';
const APPLIED_MIGRATION = 'clomonitorAppliedMigration';
export const DEFAULT_SEARCH_LIMIT = 20;
const DEFAULT_THEME = 'automatic';

interface Migration {
  key: number;
  description: string;
  method: (lsActual: PreferencesList) => PreferencesList;
}

const DEFAULT_PREFS: Prefs = {
  search: { limit: DEFAULT_SEARCH_LIMIT, sort: { by: DEFAULT_SORT_BY, direction: DEFAULT_SORT_DIRECTION } },
  theme: { configured: DEFAULT_THEME, effective: detectActiveThemeMode() },
};

const migrations: Migration[] = [
  {
    key: 1,
    description: 'Add sorting criteria',
    method: (lsActual: PreferencesList): PreferencesList => {
      let lsUpdated: PreferencesList = { ...lsActual };
      let guestPrefs: Prefs = lsUpdated.guest ? { ...lsUpdated.guest } : DEFAULT_PREFS;

      if (isUndefined(guestPrefs.search.sort)) {
        guestPrefs.search = {
          ...guestPrefs.search,
          sort: DEFAULT_PREFS.search.sort,
        };
      }
      return { ...lsUpdated, guest: { ...guestPrefs } };
    },
  },
  {
    key: 2,
    description: 'Fix error with prev migration applied',
    method: (lsActual: PreferencesList): PreferencesList => lsActual,
  },
  {
    key: 3,
    description: 'Add configured theme',
    method: (lsActual: PreferencesList): PreferencesList => {
      let lsUpdated: PreferencesList = { ...lsActual };
      let guestPrefs: Prefs = lsUpdated.guest ? { ...lsUpdated.guest } : DEFAULT_PREFS;

      if (isUndefined(guestPrefs.theme.configured)) {
        guestPrefs.theme = {
          ...guestPrefs.theme,
          configured: guestPrefs.theme.effective,
        };
      }
      return { ...lsUpdated, guest: { ...guestPrefs } };
    },
  },
];

export const applyMigrations = (lsActual: PreferencesList): PreferencesList => {
  let lsUpdated: PreferencesList = { ...lsActual };
  const lastMigration = getLastMigrationNumber();

  if (isEmpty(lsUpdated)) {
    lsUpdated = { guest: DEFAULT_PREFS };
  } else {
    const sortedMigrations: Migration[] = sortBy(migrations, 'key');
    let migrationsToApply = [...sortedMigrations];
    const migrationApplied = window.localStorage.getItem(APPLIED_MIGRATION);

    if (migrationApplied) {
      // If latest migration has been applied, we don't do anything
      if (lastMigration === parseInt(migrationApplied)) {
        migrationsToApply = [];
      } else {
        // Migrations newest than current one are applied to prefs
        migrationsToApply = sortedMigrations.filter(
          (migration: Migration) => migration.key > parseInt(migrationApplied)
        );
      }
    }

    migrationsToApply.forEach((migration: Migration) => {
      lsUpdated = migration.method(lsUpdated);
    });
  }

  // Saved last migration
  try {
    window.localStorage.setItem(LS_ITEM, JSON.stringify(lsUpdated));
    window.localStorage.setItem(APPLIED_MIGRATION, lastMigration.toString());
  } catch {
    // Incognite mode
  }
  return lsUpdated;
};

const getLastMigrationNumber = (): number => {
  const sortedMigrations = sortBy(migrations, 'key');
  return sortedMigrations[sortedMigrations.length - 1].key;
};

export class LocalStoragePreferences {
  private savedPreferences: PreferencesList = { guest: DEFAULT_PREFS };

  constructor() {
    try {
      const preferences = window.localStorage.getItem(LS_ITEM);
      if (preferences) {
        this.savedPreferences = applyMigrations(JSON.parse(preferences));
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
