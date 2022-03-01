-- Start transaction and plan tests
begin;
select plan(27);

-- Check expected extension exist
select has_extension('pgcrypto');

-- Check expected tables exist
select has_table('category');
select has_table('linter');
select has_table('maturity');
select has_table('organization');
select has_table('project');
select has_table('report');
select has_table('repository');

-- Check tables have expected columns
select columns_are('category', array[
    'category_id',
    'name'
]);
select columns_are('linter', array[
    'linter_id',
    'name',
    'display_name'
]);
select columns_are('maturity', array[
    'maturity_id',
    'name'
]);
select columns_are('organization', array[
    'organization_id',
    'name',
    'display_name',
    'description',
    'home_url',
    'logo_url',
    'created_at'
]);
select columns_are('project', array[
    'project_id',
    'name',
    'display_name',
    'description',
    'home_url',
    'logo_url',
    'devstats_url',
    'score',
    'rating',
    'created_at',
    'updated_at',
    'organization_id',
    'category_id',
    'maturity_id'
]);
select columns_are('report', array[
    'report_id',
    'data',
    'errors',
    'created_at',
    'updated_at',
    'repository_id',
    'linter_id'
]);
select columns_are('repository', array[
    'repository_id',
    'name',
    'url',
    'kind',
    'digest',
    'score',
    'created_at',
    'project_id'
]);

-- Check tables have expected indexes
select indexes_are('category', array[
    'category_pkey',
    'category_name_key'
]);
select indexes_are('linter', array[
    'linter_pkey'
]);
select indexes_are('maturity', array[
    'maturity_pkey',
    'maturity_name_key'
]);
select indexes_are('organization', array[
    'organization_pkey',
    'organization_name_key'
]);
select indexes_are('project', array[
    'project_pkey',
    'project_organization_id_name_key'
]);
select indexes_are('report', array[
    'report_pkey',
    'report_repository_id_linter_id_key'
]);
select indexes_are('repository', array[
    'repository_pkey',
    'repository_project_id_name_key'
]);

-- Check expected functions exist
-- Projects
select has_function('get_project');
select has_function('search_projects');

-- Check categories exist
select results_eq(
    'select * from category',
    $$ values
        (0, 'app definition'),
        (1, 'observability'),
        (2, 'orchestration'),
        (3, 'platform'),
        (4, 'provisioning'),
        (5, 'runtime'),
        (6, 'serverless')
    $$,
    'Categories should exist'
);

-- Check linters exist
select results_eq(
    'select * from linter',
    $$ values
        (0, 'core', 'CLOMonitor Core Linter')
    $$,
    'Linters should exist'
);

-- Check maturities exist
select results_eq(
    'select * from maturity',
    $$ values
        (0, 'graduated'),
        (1, 'incubating'),
        (2, 'sandbox')
    $$,
    'Maturities should exist'
);

-- Finish tests and rollback transaction
select * from finish();
rollback;
