-- Helper function that returns the percentage of repositories passing the
-- check provided calculated considering only those where the check was run.
create or replace function repositories_passing_check(p_category text, p_check_name text)
returns real as $$
    with reports_containing_check as (
        select data
        from report
        join repository using (repository_id)
        where (data->p_category)->p_check_name <> 'null'
    )
    select
        case when (select count(*) from reports_containing_check) > 0 then
            round(count(*)::real / (select count(*) from reports_containing_check) * 100)
        else
            0
        end
    from reports_containing_check
    where data @> format('{"%s": {"%s": {"passed": true}}}', p_category, p_check_name)::jsonb
    or data @> format('{"%s": {"%s": {"exempt": true}}}', p_category, p_check_name)::jsonb;
$$ language sql;
