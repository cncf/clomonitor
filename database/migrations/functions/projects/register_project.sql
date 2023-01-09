-- register_project registers the provided project in the database.
create or replace function register_project(p_foundation_id text, p_project jsonb)
returns void as $$
declare
    v_project_id uuid;
    v_repository jsonb;
begin
    -- Register project or update existing one
    insert into project (
        name,
        display_name,
        description,
        category,
        home_url,
        logo_url,
        logo_dark_url,
        devstats_url,
        accepted_at,
        maturity,
        digest,
        foundation_id
    ) values (
        p_project->>'name',
        p_project->>'display_name',
        p_project->>'description',
        p_project->>'category',
        p_project->>'home_url',
        p_project->>'logo_url',
        p_project->>'logo_dark_url',
        p_project->>'devstats_url',
        (p_project->>'accepted_at')::date,
        (p_project->>'maturity')::maturity,
        p_project->>'digest',
        p_foundation_id
    )
    on conflict (foundation_id, name) do update
    set
        display_name = excluded.display_name,
        description = excluded.description,
        category = excluded.category,
        home_url = excluded.home_url,
        logo_url = excluded.logo_url,
        logo_dark_url = excluded.logo_dark_url,
        devstats_url = excluded.devstats_url,
        accepted_at = excluded.accepted_at,
        maturity = excluded.maturity,
        digest = excluded.digest
    returning project_id into v_project_id;

    -- Register repositories or update existing ones
    for v_repository in select * from jsonb_array_elements(p_project->'repositories')
    loop
        insert into repository (
            name,
            url,
            check_sets,
            project_id
        ) values (
            v_repository->>'name',
            v_repository->>'url',
            (select array(select jsonb_array_elements_text(v_repository->'check_sets')))::check_set[],
            v_project_id
        )
        on conflict (project_id, url) do update
        set
            name = excluded.name,
            check_sets = excluded.check_sets,
            digest = null;
    end loop;

    -- Delete repositories that are no longer available
    delete from repository
    where project_id = v_project_id
    and url not in (
        select value->>'url'
        from jsonb_array_elements(p_project->'repositories')
    );
end
$$ language plpgsql;
