-- Helper function that returns the percentage of repositories passing the
-- check provided calculated considering only those where the check was run.
create or replace function repositories_passing_check(
    p_foundation text,
    p_category text,
    p_check_name text
)
returns real as $$
    with reports_containing_check as (
        select rp.data
        from report rp
        join repository r using (repository_id)
        join project p using (project_id)
        join organization o using (organization_id)
        where (rp.data->p_category)->p_check_name <> 'null'
        and
            case when p_foundation is not null then
            o.foundation::text = p_foundation else true end
    )
    select
        case when (select count(*) from reports_containing_check) > 0 then
            round(count(*)::real / (select count(*) from reports_containing_check) * 100)::real
        else
            0
        end
    from reports_containing_check
    where data @> format('{"%s": {"%s": {"passed": true}}}', p_category, p_check_name)::jsonb;
$$ language sql;
