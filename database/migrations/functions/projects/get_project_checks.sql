-- Return the result of the checks run on the repositories of the project
-- provided.
create or replace function get_project_checks(p_project_id uuid)
returns table(repository uuid, category text, check_id text, passed boolean) as $$
declare
    report jsonb;
begin
    for repository in
        select repository_id from repository where project_id = p_project_id
    loop
        select data into report from report where repository_id = repository;

        for category in select jsonb_object_keys(report) loop
            for check_id in select jsonb_object_keys(report->category) loop
                if report->category->check_id <> 'null' then
                    select (report->category->check_id->>'passed')::boolean into passed;

                    return next;
                end if;
            end loop;
        end loop;
    end loop;
end
$$ language plpgsql;
