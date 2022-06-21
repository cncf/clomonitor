-- Returns some stats in json format.
create or replace function get_stats(p_foundation text)
returns json as $$
    set local timezone to 'utc';

    with ratings as (
        select p.maturity, p.rating, count(*) as total
        from project p
        join organization o using (organization_id)
        where p.rating is not null
        and
            case when p_foundation is not null then
            o.foundation::text = p_foundation else true end
        group by p.maturity, p.rating
    )
    select json_strip_nulls(json_build_object(
        'generated_at', floor(extract(epoch from current_timestamp) * 1000),
        'projects', json_build_object(
            'running_total', (
                select json_agg(json_build_array(
                    floor(extract(epoch from projects_month) * 1000),
                    running_total
                ))
                from (
                    select
                        projects_month,
                        sum(total) over (order by projects_month asc) as running_total
                    from (
                        select
                            date_trunc('month', p.accepted_at) as projects_month,
                            count(*) as total
                        from project p
                        join organization o using (organization_id)
                        where p.accepted_at is not null
                        and
                            case when p_foundation is not null then
                            o.foundation::text = p_foundation else true end
                        group by date_trunc('month', p.accepted_at)
                    ) mt
                ) rt
            ),
            'accepted_distribution', (
                select json_agg(row_to_json(entry_count))
                from (
                    select
                        extract('year' from p.accepted_at) as year,
                        extract('month' from p.accepted_at) as month,
                        count(*) as total
                    from project p
                    join organization o using (organization_id)
                    where p.accepted_at is not null
                    and
                        case when p_foundation is not null then
                        o.foundation::text = p_foundation else true end
                    group by
                        extract('year' from p.accepted_at),
                        extract('month' from p.accepted_at)
                    order by year desc, month desc
                ) entry_count
            ),
            'rating_distribution', json_build_object(
                'all', (
                    select json_agg(json_build_object(rating, total))
                    from (
                        select rating, sum(total) as total
                        from ratings
                        group by rating
                        order by rating asc
                    ) rg
                ),
                'graduated', (
                    select json_agg(json_build_object(rating, total))
                    from (
                        select rating, sum(total) as total
                        from ratings where maturity = 'graduated'
                        group by rating
                        order by rating asc
                    ) rg
                ),
                'incubating', (
                    select json_agg(json_build_object(rating, total))
                    from (
                        select rating, sum(total) as total
                        from ratings where maturity = 'incubating'
                        group by rating
                        order by rating asc
                    ) rg
                ),
                'sandbox', (
                    select json_agg(json_build_object(rating, total))
                    from (
                        select rating, sum(total) as total
                        from ratings where maturity = 'sandbox'
                        group by rating
                        order by rating asc
                    ) rg
                )
            ),
            'sections_average', json_build_object(
                'all', json_build_object(
                    'documentation', (average_section_score(p_foundation, 'documentation', null)),
                    'license', (average_section_score(p_foundation, 'license', null)),
                    'best_practices', (average_section_score(p_foundation, 'best_practices', null)),
                    'security', (average_section_score(p_foundation, 'security', null)),
                    'legal', (average_section_score(p_foundation, 'legal', null))
                ),
                'graduated', json_build_object(
                    'documentation', (average_section_score(p_foundation, 'documentation', 'graduated')),
                    'license', (average_section_score(p_foundation, 'license', 'graduated')),
                    'best_practices', (average_section_score(p_foundation, 'best_practices', 'graduated')),
                    'security', (average_section_score(p_foundation, 'security', 'graduated')),
                    'legal', (average_section_score(p_foundation, 'legal', 'graduated'))
                ),
                'incubating', json_build_object(
                    'documentation', (average_section_score(p_foundation, 'documentation', 'incubating')),
                    'license', (average_section_score(p_foundation, 'license', 'incubating')),
                    'best_practices', (average_section_score(p_foundation, 'best_practices', 'incubating')),
                    'security', (average_section_score(p_foundation, 'security', 'incubating')),
                    'legal', (average_section_score(p_foundation, 'legal', 'incubating'))
                ),
                'sandbox', json_build_object(
                    'documentation', (average_section_score(p_foundation, 'documentation', 'sandbox')),
                    'license', (average_section_score(p_foundation, 'license', 'sandbox')),
                    'best_practices', (average_section_score(p_foundation, 'best_practices', 'sandbox')),
                    'security', (average_section_score(p_foundation, 'security', 'sandbox')),
                    'legal', (average_section_score(p_foundation, 'legal', 'sandbox'))
                )
            )
        ),
        'repositories', json_build_object(
            'passing_check', json_build_object(
                'documentation', json_build_object(
                    'adopters', repositories_passing_check(p_foundation, 'documentation', 'adopters'),
                    'changelog', repositories_passing_check(p_foundation, 'documentation', 'changelog'),
                    'code_of_conduct', repositories_passing_check(p_foundation, 'documentation', 'code_of_conduct'),
                    'contributing', repositories_passing_check(p_foundation, 'documentation', 'contributing'),
                    'governance', repositories_passing_check(p_foundation, 'documentation', 'governance'),
                    'maintainers', repositories_passing_check(p_foundation, 'documentation', 'maintainers'),
                    'readme', repositories_passing_check(p_foundation, 'documentation', 'readme'),
                    'roadmap', repositories_passing_check(p_foundation, 'documentation', 'roadmap'),
                    'website', repositories_passing_check(p_foundation, 'documentation', 'website')
                ),
                'license', json_build_object(
                    'approved', repositories_passing_check(p_foundation, 'license', 'approved'),
                    'scanning', repositories_passing_check(p_foundation, 'license', 'scanning'),
                    'spdx_id', repositories_passing_check(p_foundation, 'license', 'spdx_id')
                ),
                'best_practices', json_build_object(
                    'artifacthub_badge', repositories_passing_check(p_foundation, 'best_practices', 'artifacthub_badge'),
                    'cla', repositories_passing_check(p_foundation, 'best_practices', 'cla'),
                    'community_meeting', repositories_passing_check(p_foundation, 'best_practices', 'community_meeting'),
                    'dco', repositories_passing_check(p_foundation, 'best_practices', 'dco'),
                    'ga4', repositories_passing_check(p_foundation, 'best_practices', 'ga4'),
                    'openssf_badge', repositories_passing_check(p_foundation, 'best_practices', 'openssf_badge'),
                    'recent_release', repositories_passing_check(p_foundation, 'best_practices', 'recent_release'),
                    'slack_presence', repositories_passing_check(p_foundation, 'best_practices', 'slack_presence')
                ),
                'security', json_build_object(
                    'binary_artifacts', repositories_passing_check(p_foundation, 'security', 'binary_artifacts'),
                    'branch_protection', repositories_passing_check(p_foundation, 'security', 'branch_protection'),
                    'code_review', repositories_passing_check(p_foundation, 'security', 'code_review'),
                    'dangerous_workflow', repositories_passing_check(p_foundation, 'security', 'dangerous_workflow'),
                    'dependency_update_tool', repositories_passing_check(p_foundation, 'security', 'dependency_update_tool'),
                    'maintained', repositories_passing_check(p_foundation, 'security', 'maintained'),
                    'sbom', repositories_passing_check(p_foundation, 'security', 'sbom'),
                    'security_policy', repositories_passing_check(p_foundation, 'security', 'security_policy'),
                    'signed_releases', repositories_passing_check(p_foundation, 'security', 'signed_releases'),
                    'token_permissions', repositories_passing_check(p_foundation, 'security', 'token_permissions'),
                    'vulnerabilities', repositories_passing_check(p_foundation, 'security', 'vulnerabilities')
                ),
                'legal', json_build_object(
                    'trademark_disclaimer', repositories_passing_check(p_foundation, 'legal', 'trademark_disclaimer')
                )
            )
        )
    ));
$$ language sql;
