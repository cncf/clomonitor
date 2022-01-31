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

create table if not exists repository (
    repository_id uuid primary key default gen_random_uuid(),
    name text not null check (name <> ''),
    url text not null check (url <> ''),
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

insert into linter (linter_id, name, display_name) values (0, 'core', 'Remonitor Core Linter');

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

create or replace function search_projects(p_input jsonb)
returns table(projects json, total_count bigint) as $$
declare
    v_text text := (p_input->>'text');
    v_category int[];
    v_maturity int[];
    v_rating text[];
begin
    -- Prepare filters
    if p_input ? 'category' and p_input->'category' <> 'null' then
        select array_agg(e::int) into v_category
        from jsonb_array_elements_text(p_input->'category') e;
    end if;
    if p_input ? 'maturity' and p_input->'maturity' <> 'null' then
        select array_agg(e::int) into v_maturity
        from jsonb_array_elements_text(p_input->'maturity') e;
    end if;
    if p_input ? 'rating' and p_input->'rating' <> 'null' then
        select array_agg(e::text) into v_rating
        from jsonb_array_elements_text(p_input->'rating') e;
    end if;

    return query
    with filtered_projects as (
        select
            p.project_id,
            p.name,
            p.display_name,
            p.description,
            p.home_url,
            coalesce(p.logo_url, o.logo_url) as logo_url,
            p.devstats_url,
            p.score,
            p.rating,
            p.category_id,
            p.maturity_id,
            p.updated_at
        from project p
        join organization o using (organization_id)
        where score is not null
        and
            case when v_text is not null then p.name ~* v_text else true end
        and
            case when cardinality(v_category) > 0
            then p.category_id = any(v_category) else true end
        and
            case when cardinality(v_maturity) > 0
            then p.maturity_id = any(v_maturity) else true end
        and
            case when cardinality(v_rating) > 0
            then p.rating = any(v_rating) else true end
    )
    select
        (
            select coalesce(json_agg(json_strip_nulls(json_build_object(
                'id', project_id,
                'name', name,
                'display_name', display_name,
                'description', description,
                'home_url', home_url,
                'logo_url', logo_url,
                'devstats_url', devstats_url,
                'score', score,
                'rating', rating,
                'category_id', category_id,
                'maturity_id', maturity_id,
                'updated_at', floor(extract(epoch from updated_at)),
                'repositories', (
                    select json_agg(json_build_object(
                        'name', name,
                        'url', url
                    ))
                    from repository
                    where project_id = fpp.project_id
        )
            ))), '[]')
            from (
                select *
                from filtered_projects
                order by name asc
                limit (p_input->>'limit')::int
                offset (p_input->>'offset')::int
            ) fpp
        ),
        (
            select count(*) from filtered_projects
        );
end
$$ language plpgsql;

create or replace function get_project(p_project_id uuid)
returns json as $$
    select json_strip_nulls(json_build_object(
        'id', p.project_id,
        'name', p.name,
        'display_name', p.display_name,
        'description', p.description,
        'home_url', p.home_url,
        'logo_url', coalesce(p.logo_url, o.logo_url),
        'devstats_url', p.devstats_url,
        'score', p.score,
        'rating', p.rating,
        'category_id', p.category_id,
        'maturity_id', p.maturity_id,
        'updated_at', floor(extract(epoch from p.updated_at)),
        'repositories', (
            select json_agg(json_build_object(
                'repository_id', r.repository_id,
                'name', r.name,
                'url', r.url,
                'digest', r.digest,
                'score', r.score,
                'reports', (
                    select json_agg(json_build_object(
                        'report_id', report_id,
                        'data', data,
                        'errors', errors,
                        'updated_at', floor(extract(epoch from updated_at)),
                        'linter_id', linter_id
                    ))
                    from report
                    where repository_id = r.repository_id
                )
            ))
            from repository r
            where project_id = p_project_id
        )
    ))
    from project p
    join organization o using (organization_id)
    where project_id = p_project_id;
$$ language sql;
