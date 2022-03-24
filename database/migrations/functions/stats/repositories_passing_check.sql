-- Helper function that returns the percentage of repositories of kind primary
-- passing the check provided.
create or replace function repositories_passing_check(p_category text, p_check_name text)
returns real as $$
    with primary_repositories_reports as (
        select data
        from report
        join repository using (repository_id)
        where repository.kind = 'primary'
        and report.linter_id = 0
    )
    select
        case when (select count(*) from primary_repositories_reports) > 0 then
            round(count(*)::real / (select count(*) from primary_repositories_reports) * 100)
        else
            0
        end
    from primary_repositories_reports
    where data @> format('{"%s": {"%s": {"passed": true}}}', p_category, p_check_name)::jsonb
    or data @> format('{"%s": {"%s": {"exempt": true}}}', p_category, p_check_name)::jsonb;
$$ language sql;
