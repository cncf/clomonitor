create or replace function get_project(p_org_name text, p_project_name text)
returns json as $$
    select json_strip_nulls(json_build_object(
        'id', p.project_id,
        'name', p.name,
        'display_name', p.display_name,
        'description', p.description,
        'home_url', p.home_url,
        'logo_url', coalesce(p.logo_url, o.logo_url),
        'devstats_url', p.devstats_url,
        'score', p.score,
        'rating', p.rating,
        'category_id', p.category_id,
        'maturity_id', p.maturity_id,
        'accepted_at', extract(epoch from p.accepted_at),
        'updated_at', floor(extract(epoch from p.updated_at)),
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
        )
    ))
    from project p
    join organization o using (organization_id)
    where o.name = p_org_name and p.name = p_project_name;
$$ language sql;
