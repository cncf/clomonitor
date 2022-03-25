-- get_stats returns some stats formatted as json.
create or replace function get_stats()
returns json as $$
    set local timezone to 'utc';

    with ratings as (
        select maturity_id, rating, count(*) as total
        from project
        where rating is not null
        group by maturity_id, rating
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
                            date_trunc('month', accepted_at) as projects_month,
                            count(*) as total
                        from project
                        where accepted_at is not null
                        group by date_trunc('month', accepted_at)
                    ) mt
                ) rt
            ),
            'accepted_distribution', (
                select json_agg(row_to_json(entry_count))
                from (
                    select
                        extract('year' from accepted_at) as year,
                        extract('month' from accepted_at) as month,
                        count(*) as total
                    from project
                    where accepted_at is not null
                    group by
                        extract('year' from accepted_at),
                        extract('month' from accepted_at)
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
                    ) rg
                ),
                'graduated', (
                    select json_agg(json_build_object(rating, total))
                    from (
                        select rating, sum(total) as total
                        from ratings where maturity_id = 0 group by rating
                    ) rg
                ),
                'incubating', (
                    select json_agg(json_build_object(rating, total))
                    from (
                        select rating, sum(total) as total
                        from ratings where maturity_id = 1 group by rating
                    ) rg
                ),
                'sandbox', (
                    select json_agg(json_build_object(rating, total))
                    from (
                        select rating, sum(total) as total
                        from ratings where maturity_id = 2 group by rating
                    ) rg
                )
            ),
            'sections_average', json_build_object(
                'all', json_build_object(
                    'documentation', (average_section_score('documentation', null)),
                    'license', (average_section_score('license', null)),
                    'best_practices', (average_section_score('best_practices', null)),
                    'security', (average_section_score('security', null)),
                    'legal', (average_section_score('legal', null))
                ),
                'graduated', json_build_object(
                    'documentation', (average_section_score('documentation', 0)),
                    'license', (average_section_score('license', 0)),
                    'best_practices', (average_section_score('best_practices', 0)),
                    'security', (average_section_score('security', 0)),
                    'legal', (average_section_score('legal', 0))
                ),
                'incubating', json_build_object(
                    'documentation', (average_section_score('documentation', 1)),
                    'license', (average_section_score('license', 1)),
                    'best_practices', (average_section_score('best_practices', 1)),
                    'security', (average_section_score('security', 1)),
                    'legal', (average_section_score('legal', 1))
                ),
                'sandbox', json_build_object(
                    'documentation', (average_section_score('documentation', 2)),
                    'license', (average_section_score('license', 2)),
                    'best_practices', (average_section_score('best_practices', 2)),
                    'security', (average_section_score('security', 2)),
                    'legal', (average_section_score('legal', 2))
                )
            )
        ),
        'repositories', json_build_object(
            'passing_check', json_build_object(
                'documentation', json_build_object(
                    'adopters', repositories_passing_check('documentation', 'adopters'),
                    'changelog', repositories_passing_check('documentation', 'changelog'),
                    'code_of_conduct', repositories_passing_check('documentation', 'code_of_conduct'),
                    'contributing', repositories_passing_check('documentation', 'contributing'),
                    'governance', repositories_passing_check('documentation', 'governance'),
                    'maintainers', repositories_passing_check('documentation', 'maintainers'),
                    'readme', repositories_passing_check('documentation', 'readme'),
                    'roadmap', repositories_passing_check('documentation', 'roadmap'),
                    'website', repositories_passing_check('documentation', 'website')
                ),
                'license', json_build_object(
                    'approved', repositories_passing_check('license', 'approved'),
                    'scanning', repositories_passing_check('license', 'scanning'),
                    'spdx_id', repositories_passing_check('license', 'spdx_id')
                ),
                'best_practices', json_build_object(
                    'artifacthub_badge', repositories_passing_check('best_practices', 'artifacthub_badge'),
                    'community_meeting', repositories_passing_check('best_practices', 'community_meeting'),
                    'dco', repositories_passing_check('best_practices', 'dco'),
                    'openssf_badge', repositories_passing_check('best_practices', 'openssf_badge'),
                    'recent_release', repositories_passing_check('best_practices', 'recent_release'),
                    'slack_presence', repositories_passing_check('best_practices', 'slack_presence')
                ),
                'security', json_build_object(
                    'sbom', repositories_passing_check('security', 'sbom'),
                    'security_policy', repositories_passing_check('security', 'security_policy')
                ),
                'legal', json_build_object(
                    'trademark_disclaimer', repositories_passing_check('legal', 'trademark_disclaimer')
                )
            )
        )
    ));
$$ language sql;
