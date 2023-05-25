-- Returns some information about the pending annual review notifications.
create or replace function get_pending_annual_review_notifications()
returns table(
    project_id uuid,
    community_repo_url text,
    issue_number bigint
) as $$
    select distinct on (project_id)
        project_id,
        r.url as community_repo_url,
        last_initial_notification.issue_number
    from repository r
    join report rp using (repository_id)
    join project p using (project_id)
    left join (
        select
            distinct on (project_id)
            project_id,
            created_at
        from annual_review_notification
        order by project_id, created_at desc
    ) last_notification using (project_id)
    left join (
        select
            distinct on (project_id)
            project_id,
            issue_number
        from annual_review_notification
        where issue_number is not null
        and comment_id is null
        and current_timestamp - created_at < '1 year'::interval
        order by project_id, created_at desc
    ) last_initial_notification using (project_id)
    where
        'community' = any(r.check_sets)
        and rp.data @>
            '{
                "documentation": {
                    "annual_review": {
                        "passed": false,
                        "exempt": false,
                        "failed": false
                    }
                }
            }'
        and (
            last_notification.created_at is null
            or current_timestamp - last_notification.created_at > '1 month'::interval
        );
$$ language sql;
