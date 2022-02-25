import { isUndefined } from 'lodash';

import { Prefs, SortBy, SortDirection } from '../types';
import lsPreferences, { applyMigrations, PreferencesList } from './localStoragePreferences';

const defaultPrefs: Prefs = {
  search: { limit: 20, sort: { by: SortBy.Name, direction: SortDirection.ASC } },
  theme: {
    effective: 'light',
  },
};

interface ApplyMigrationsTests {
  appliedMigration?: string;
  input: any;
  output: PreferencesList;
}

const applyMigrationsTests: ApplyMigrationsTests[] = [
  {
    input: {},
    output: { guest: { ...defaultPrefs } },
  },
  {
    appliedMigration: '0',
    input: {
      guest: {
        search: { limit: 20 },
        theme: {
          effective: 'light',
        },
      },
    },
    output: {
      guest: defaultPrefs,
    },
  },
  {
    appliedMigration: '0',
    input: {
      guest: {
        search: { limit: 20 },
        theme: {
          effective: 'dark',
        },
      },
    },
    output: {
      guest: { ...defaultPrefs, theme: { effective: 'dark' } },
    },
  },
];

describe('localStoragePreferences', () => {
  afterAll(() => {
    window.localStorage.removeItem('clomonitorPrefs');
  });

  it('init LocalStoragePreferences', () => {
    expect(lsPreferences.getPrefs()).toStrictEqual(defaultPrefs);
  });

  it('saves prefs', () => {
    lsPreferences.setPrefs(defaultPrefs);
    expect(lsPreferences.getPrefs()).toStrictEqual(defaultPrefs);
  });

  it('updates prefs', () => {
    expect(lsPreferences.getPrefs()).toStrictEqual(defaultPrefs);
    const newPrefs: Prefs = {
      ...defaultPrefs,
      theme: { effective: 'dark' },
    };
    lsPreferences.setPrefs(newPrefs);
    expect(lsPreferences.getPrefs()).toStrictEqual(newPrefs);
  });

  describe('Apply migrations', () => {
    for (let i = 0; i < applyMigrationsTests.length; i++) {
      it('get correct Prefs', () => {
        if (!isUndefined(applyMigrationsTests[i].appliedMigration)) {
          window.localStorage.setItem('clomonitorAppliedMigration', applyMigrationsTests[i].appliedMigration!);
        }
        const prefs = applyMigrations(applyMigrationsTests[i].input);
        expect(prefs).toEqual(applyMigrationsTests[i].output);
      });
    }
  });
});
