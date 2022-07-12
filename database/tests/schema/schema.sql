-- Start transaction and plan tests
begin;
select plan(20);

-- Check expected extension exist
select has_extension('pgcrypto');

-- Check expected tables exist
select has_table('organization');
select has_table('project');
select has_table('report');
select has_table('repository');

-- Check tables have expected columns
select columns_are('organization', array[
    'organization_id',
    'name',
    'display_name',
    'description',
    'home_url',
    'logo_url',
    'created_at',
    'foundation'
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
    'organization_id'
]);
select columns_are('report', array[
    'report_id',
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
select indexes_are('organization', array[
    'organization_pkey',
    'organization_foundation_name_key'
]);
select indexes_are('project', array[
    'project_pkey',
    'project_organization_id_name_key'
]);
select indexes_are('report', array[
    'report_pkey',
    'report_repository_id_key'
]);
select indexes_are('repository', array[
    'repository_pkey',
    'repository_project_id_name_key'
]);

-- Check expected functions exist
-- Projects
select has_function('get_project');
select has_function('get_project_checks');
select has_function('get_project_passed_checks');
select has_function('search_projects');
-- Repositories
select has_function('get_repositories_with_checks');
-- Stats
select has_function('repositories_passing_check');
select has_function('get_stats');

-- Finish tests and rollback transaction
select * from finish();
rollback;
