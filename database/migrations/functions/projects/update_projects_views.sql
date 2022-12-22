-- update_projects_views updates the views of the projects provided.
create or replace function update_projects_views(p_lock_key bigint, p_data jsonb)
returns void as $$
    -- Make sure only one batch of updates is processed at a time
    select pg_advisory_xact_lock(p_lock_key);

    -- Insert or update the corresponding views counters as needed
    insert into project_views (project_id, day, total)
    select
        (value->>0)::uuid as project_id,
        (value->>1)::date as day,
        (value->>2)::integer as total
    from jsonb_array_elements(p_data)
    on conflict (project_id, day) do
    update set total = project_views.total + excluded.total;
$$ language sql;
