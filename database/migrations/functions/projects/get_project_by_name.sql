-- Returns some information about a project in json format.
create or replace function get_project_by_name(
    p_foundation text,
    p_project_name text
)
returns json as $$
    select get_project_by_id(p.project_id)
    from project p
    where p.foundation_id = p_foundation
    and p.name = p_project_name;
$$ language sql;
