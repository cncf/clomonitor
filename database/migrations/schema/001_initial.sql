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

create table if not exists category (
    category_id integer primary key,
    name text not null check (name <> '') unique
);

insert into category (category_id, name) values (0, 'app definition');
insert into category (category_id, name) values (1, 'observability');
insert into category (category_id, name) values (2, 'orchestration');
insert into category (category_id, name) values (3, 'platform');
insert into category (category_id, name) values (4, 'provisioning');
insert into category (category_id, name) values (5, 'runtime');
insert into category (category_id, name) values (6, 'serverless');

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
    score jsonb,
    rating text,
    created_at timestamptz default current_timestamp not null,
    updated_at timestamptz default current_timestamp not null,
    organization_id uuid not null references organization on delete cascade,
    category_id integer not null references category on delete restrict,
    maturity_id integer not null references maturity on delete restrict,
    unique (organization_id, name)
);

create type repository_kind as enum ('primary', 'secondary');

create table if not exists repository (
    repository_id uuid primary key default gen_random_uuid(),
    name text not null check (name <> ''),
    url text not null check (url <> ''),
    kind repository_kind not null,
    digest text,
    score jsonb,
    created_at timestamptz default current_timestamp not null,
    project_id uuid not null references project on delete cascade,
    unique (project_id, name)
);

create table if not exists linter (
    linter_id integer primary key,
    name text not null check (name <> ''),
    display_name text check (display_name <> '')
);

insert into linter (linter_id, name, display_name) values (0, 'core', 'CLOMonitor Core Linter');

create table if not exists report (
    report_id uuid primary key default gen_random_uuid(),
    data jsonb,
    errors text,
    created_at timestamptz default current_timestamp not null,
    updated_at timestamptz default current_timestamp not null,
    repository_id uuid not null references repository on delete cascade,
    linter_id integer not null references linter on delete restrict,
    unique (repository_id, linter_id)
);
