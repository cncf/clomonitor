-- Returns some information about a repository in json format.
create or replace function get_repository_report(
    p_foundation text,
    p_project_name text,
    p_repository_name text
)
returns json as $$
    select json_strip_nulls(json_build_object(
        'name', repo.name,
        'url', repo.url,
        'check_sets', report.check_sets,
        'score', repo.score,
        'report', report.data
    ))
    from repository repo
    join project p using (project_id)
    join report report using (repository_id)
    where p.foundation_id = p_foundation
    and p.name = p_project_name
    and repo.name = p_repository_name;
$$ language sql;
