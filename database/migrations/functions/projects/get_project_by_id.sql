-- Returns some information about a project in json format.
create or replace function get_project_by_id(p_project_id uuid)
returns json as $$
    select json_strip_nulls(json_build_object(
        'id', p.project_id,
        'name', p.name,
        'display_name', p.display_name,
        'description', p.description,
        'category', p.category,
        'home_url', p.home_url,
        'logo_url', p.logo_url,
        'devstats_url', p.devstats_url,
        'score', p.score,
        'rating', p.rating,
        'accepted_at', extract(epoch from p.accepted_at),
        'updated_at', floor(extract(epoch from p.updated_at)),
        'maturity', p.maturity,
        'repositories', (
            select json_agg(json_build_object(
                'repository_id', r.repository_id,
                'name', r.name,
                'url', r.url,
                'check_sets', r.check_sets,
                'digest', r.digest,
                'score', r.score,
                'report', (
                    select json_build_object(
                        'report_id', report_id,
                        'check_sets', check_sets,
                        'data', data,
                        'errors', errors,
                        'updated_at', floor(extract(epoch from updated_at))
                    )
                    from report
                    where repository_id = r.repository_id
                )
            ))
            from repository r
            where project_id = p.project_id
        ),
        'snapshots', (
            select json_agg(s.date)
            from (
                select date
                from project_snapshot
                where project_id = p_project_id
                order by date desc
            ) s
        ),
        'foundation', p.foundation_id
    ))
    from project p
    where p.project_id = p_project_id
    and p.score is not null;
$$ language sql;
