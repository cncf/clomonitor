{{ template "projects/get_project.sql" }}
{{ template "projects/get_project_checks.sql" }}
{{ template "projects/get_project_passed_checks.sql" }}
{{ template "projects/search_projects.sql" }}
{{ template "repositories/get_repositories_with_checks.sql" }}
{{ template "stats/average_section_score.sql" }}
{{ template "stats/repositories_passing_check.sql" }}
{{ template "stats/get_stats.sql" }}

---- create above / drop below ----

-- Nothing to do
