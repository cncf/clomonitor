create extension if not exists pgcrypto;

create table if not exists organization (
    organization_id uuid primary key default gen_random_uuid(),
    name text not null check (name <> '') unique,
    display_name text check (display_name <> ''),
    description text check (description <> ''),
    home_url text check (home_url <> ''),
    logo_url text check (logo_url <> ''),
    created_at timestamptz default current_timestamp not null
);

create table if not exists maturity (
    maturity_id integer primary key,
    name text not null check (name <> '') unique
);

insert into maturity (maturity_id, name) values (0, 'graduated');
insert into maturity (maturity_id, name) values (1, 'incubating');
insert into maturity (maturity_id, name) values (2, 'sandbox');

create table if not exists project (
    project_id uuid primary key default gen_random_uuid(),
    name text not null check (name <> ''),
    display_name text check (display_name <> ''),
    description text check (description <> ''),
    home_url text check (home_url <> ''),
    logo_url text check (logo_url <> ''),
    devstats_url text check (devstats_url <> ''),
    created_at timestamptz default current_timestamp not null,
    organization_id uuid references organization on delete cascade,
    maturity_id integer references maturity on delete restrict,
    unique (organization_id, name)
);

create table if not exists repository (
    repository_id uuid primary key default gen_random_uuid(),
    name text not null check (name <> ''),
    url text not null check (url <> ''),
    created_at timestamptz default current_timestamp not null,
    project_id uuid references project on delete cascade,
    unique (project_id, name)
);

create table if not exists linter (
    linter_id integer primary key,
    name text not null check (name <> ''),
    display_name text check (display_name <> '')
);

insert into linter (linter_id, name, display_name) values (0, 'core', 'Remonitor Core Linter');

create table if not exists report (
    report_id uuid primary key default gen_random_uuid(),
    data jsonb not null,
    created_at timestamptz default current_timestamp not null,
    repository_id uuid references repository on delete cascade,
    linter_id integer references linter on delete restrict,
    unique (repository_id, linter_id)
);

-- Load sample data
copy organization (organization_id, name, home_url, logo_url)
from '../../projects/remonitor/database/data/organizations.csv'
with (format csv, header true);
copy project (project_id, maturity_id, name, description, organization_id)
from '../../projects/remonitor/database/data/projects.csv'
with (format csv, header true);
copy repository (repository_id, name, url, project_id)
from '../../projects/remonitor/database/data/repositories.csv'
with (format csv, header true);
