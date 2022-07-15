create extension if not exists pgcrypto;

create type foundation as enum ('cncf', 'lfaidata');

create table if not exists organization (
    organization_id uuid primary key default gen_random_uuid(),
    name text not null check (name <> ''),
    display_name text check (display_name <> ''),
    description text check (description <> ''),
    home_url text check (home_url <> ''),
    logo_url text check (logo_url <> ''),
    created_at timestamptz default current_timestamp not null,
    foundation foundation not null,
    unique (foundation, name)
);

create type maturity as enum ('graduated', 'incubating', 'sandbox');

create table if not exists project (
    project_id uuid primary key default gen_random_uuid(),
    name text not null check (name <> ''),
    display_name text check (display_name <> ''),
    description text check (description <> ''),
    category text check (category <> ''),
    home_url text check (home_url <> ''),
    logo_url text check (logo_url <> ''),
    devstats_url text check (devstats_url <> ''),
    score jsonb,
    rating text,
    passed_checks text[],
    accepted_at date,
    created_at timestamptz default current_timestamp not null,
    updated_at timestamptz default current_timestamp not null,
    maturity maturity not null,
    organization_id uuid not null references organization on delete cascade,
    unique (organization_id, name)
);

create type check_set as enum ('code', 'code-lite', 'community', 'docs');

create table if not exists repository (
    repository_id uuid primary key default gen_random_uuid(),
    name text not null check (name <> ''),
    url text not null check (url <> ''),
    digest text,
    score jsonb,
    created_at timestamptz default current_timestamp not null,
    updated_at timestamptz default current_timestamp not null,
    check_sets check_set[] not null,
    project_id uuid not null references project on delete cascade,
    unique (project_id, name)
);

create table if not exists report (
    report_id uuid primary key default gen_random_uuid(),
    data jsonb,
    errors text,
    created_at timestamptz default current_timestamp not null,
    updated_at timestamptz default current_timestamp not null,
    repository_id uuid not null references repository on delete cascade,
    unique (repository_id)
);
