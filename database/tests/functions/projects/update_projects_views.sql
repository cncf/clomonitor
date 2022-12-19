-- Start transaction and plan tests
begin;
select plan(3);

-- Declare some variables
\set lockKey 1

-- Seed some data
insert into foundation values ('cncf', 'CNCF', 'http://127.0.0.1:8080/cncf.yaml');
insert into project (
    project_id,
    name,
    category,
    accepted_at,
    maturity,
    foundation_id
) values (
    '00000000-0000-0000-0000-000000000001',
    'project1',
    'category1',
    '2022-12-19',
    'sandbox',
    'cncf'
);
insert into project (
    project_id,
    name,
    category,
    accepted_at,
    maturity,
    foundation_id
) values (
    '00000000-0000-0000-0000-000000000002',
    'project2',
    'category1',
    '2022-12-19',
    'incubating',
    'cncf'
);

-- Run some tests
select update_projects_views(:lockKey, '[
    ["00000000-0000-0000-0000-000000000001", "2022-12-19", 10]
]');
select results_eq(
    'select * from project_views',
    $$ values
        ('00000000-0000-0000-0000-000000000001'::uuid, '2022-12-19'::date, 10)
    $$,
    'First run: one insert'
);
select update_projects_views(:lockKey, '[
    ["00000000-0000-0000-0000-000000000001", "2022-12-19", 10]
]');
select results_eq(
    'select * from project_views',
    $$ values
        ('00000000-0000-0000-0000-000000000001'::uuid, '2022-12-19'::date, 20)
    $$,
    'Second run: one update'
);
select update_projects_views(:lockKey, '[
    ["00000000-0000-0000-0000-000000000001", "2022-12-19", 10],
    ["00000000-0000-0000-0000-000000000001", "2022-12-20", 10],
    ["00000000-0000-0000-0000-000000000002", "2022-12-20", 10],
    ["00000000-0000-0000-0000-000000000002", "2022-12-21", 5]
]');
select results_eq(
    'select * from project_views',
    $$ values
        ('00000000-0000-0000-0000-000000000001'::uuid, '2022-12-19'::date, 30),
        ('00000000-0000-0000-0000-000000000001'::uuid, '2022-12-20'::date, 10),
        ('00000000-0000-0000-0000-000000000002'::uuid, '2022-12-20'::date, 10),
        ('00000000-0000-0000-0000-000000000002'::uuid, '2022-12-21'::date, 5)
    $$,
    'Third run: one update and three inserts'
);

-- Finish tests and rollback transaction
select * from finish();
rollback;
