-- Return passed checks on the repositories of the project provided.
-- For checks run on multiple repositories, we considered the check to have
-- passed if it passes on all repositories where it was run.
create or replace function get_project_passed_checks(p_project_id uuid)
returns text[] as $$
    select array(
        select check_id
        from get_project_checks(p_project_id)
        group by check_id
        having sum(case when passed <> false then 0 else 1 end) = 0
        order by check_id asc
    );
$$ language sql;
