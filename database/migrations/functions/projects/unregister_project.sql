-- unregister_project unregisters the provided project from the database.
create or replace function unregister_project(p_foundation_id text, p_project_name text)
returns void as $$
    delete from project
    where foundation_id = p_foundation_id
    and name = p_project_name;
$$ language sql;
