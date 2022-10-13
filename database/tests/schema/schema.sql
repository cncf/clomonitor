-- Start transaction and plan tests
begin;
select plan(23);

-- Check expected extension exist
select has_extension('pgcrypto');

-- Check expected tables exist
select has_table('foundation');
select has_table('project');
select has_table('report');
select has_table('repository');

-- Check tables have expected columns
select columns_are('foundation', array[
    'foundation_id',
    'display_name',
    'data_url'
]);
select columns_are('project', array[
    'project_id',
    'name',
    'display_name',
    'description',
    'category',
    'home_url',
    'logo_url',
    'devstats_url',
    'score',
    'rating',
    'passed_checks',
    'accepted_at',
    'created_at',
    'updated_at',
    'maturity',
    'digest',
    'foundation_id'
]);
select columns_are('report', array[
    'report_id',
    'check_sets',
    'data',
    'errors',
    'created_at',
    'updated_at',
    'repository_id'
]);
select columns_are('repository', array[
    'repository_id',
    'name',
    'url',
    'digest',
    'score',
    'created_at',
    'updated_at',
    'check_sets',
    'project_id'
]);

-- Check tables have expected indexes
select indexes_are('foundation', array[
    'foundation_pkey'
]);
select indexes_are('project', array[
    'project_pkey',
    'project_foundation_id_name_key'
]);
select indexes_are('report', array[
    'report_pkey',
    'report_repository_id_key'
]);
select indexes_are('repository', array[
    'repository_pkey',
    'repository_project_id_url_key'
]);

-- Check expected functions exist
-- Projects
select has_function('get_project');
select has_function('get_project_checks');
select has_function('get_project_passed_checks');
select has_function('register_project');
select has_function('search_projects');
select has_function('unregister_project');
-- Repositories
select has_function('get_repositories_with_checks');
select has_function('get_repository_report');
-- Stats
select has_function('repositories_passing_check');
select has_function('get_stats');

-- Finish tests and rollback transaction
select * from finish();
rollback;
