-- Start transaction and plan tests
begin;
select plan(1);

-- Seed some data
insert into foundation values ('cncf', 'CNCF', 'http://127.0.0.1:8080/cncf.yaml');

-- Project 1
insert into project (
    project_id,
    name,
    foundation_id
) values (
    '00000000-0001-0000-0000-000000000000',
    'project1',
    'cncf'
);
insert into repository (
    repository_id,
    name,
    url,
    check_sets,
    project_id
) values (
    '00000000-0000-0001-0000-000000000000',
    'repository1',
    'https://repo1.url',
    '{code,community}',
    '00000000-0001-0000-0000-000000000000'
);
insert into report (
    report_id,
    data,
    repository_id
) values (
    '00000000-0000-0000-0001-000000000000',
    '{
        "documentation": {
            "annual_review": {
                "passed": false,
                "exempt": false,
                "failed": false
            }
        }
    }',
    '00000000-0000-0001-0000-000000000000'
);

-- Project 2
insert into project (
    project_id,
    name,
    foundation_id
) values (
    '00000000-0002-0000-0000-000000000000',
    'project2',
    'cncf'
);
insert into repository (
    repository_id,
    name,
    url,
    check_sets,
    project_id
) values (
    '00000000-0000-0002-0000-000000000000',
    'repository2',
    'https://repo2.url',
    '{code,community}',
    '00000000-0002-0000-0000-000000000000'
);
insert into report (
    report_id,
    data,
    repository_id
) values (
    '00000000-0000-0000-0002-000000000000',
    '{
        "documentation": {
            "annual_review": {
                "passed": true
            }
        }
    }',
    '00000000-0000-0002-0000-000000000000'
);

-- Project 3
insert into project (
    project_id,
    name,
    foundation_id
) values (
    '00000000-0003-0000-0000-000000000000',
    'project3',
    'cncf'
);
insert into repository (
    repository_id,
    name,
    url,
    check_sets,
    project_id
) values (
    '00000000-0000-0003-0000-000000000000',
    'repository3',
    'https://repo3.url',
    '{code,community}',
    '00000000-0003-0000-0000-000000000000'
);
insert into report (
    report_id,
    data,
    repository_id
) values (
    '00000000-0000-0000-0003-000000000000',
    '{
        "documentation": {
            "annual_review": {
                "passed": false,
                "exempt": false,
                "failed": false
            }
        }
    }',
    '00000000-0000-0003-0000-000000000000'
);
insert into annual_review_notification (
    repository_url,
    created_at,
    project_id
) values (
    'https://repo3.url',
    current_timestamp - '1 day'::interval,
    '00000000-0003-0000-0000-000000000000'
);
insert into annual_review_notification (
    repository_url,
    created_at,
    project_id
) values (
    'https://repo3.url',
    current_timestamp - '7 day'::interval,
    '00000000-0003-0000-0000-000000000000'
);

-- Project 4
insert into project (
    project_id,
    name,
    foundation_id
) values (
    '00000000-0004-0000-0000-000000000000',
    'project4',
    'cncf'
);
insert into repository (
    repository_id,
    name,
    url,
    check_sets,
    project_id
) values (
    '00000000-0000-0004-0000-000000000000',
    'repository4',
    'https://repo4.url',
    '{code,community}',
    '00000000-0004-0000-0000-000000000000'
);
insert into report (
    report_id,
    data,
    repository_id
) values (
    '00000000-0000-0000-0004-000000000000',
    '{
        "documentation": {
            "annual_review": {
                "passed": false,
                "exempt": false,
                "failed": false
            }
        }
    }',
    '00000000-0000-0004-0000-000000000000'
);
insert into annual_review_notification (
    repository_url,
    created_at,
    project_id
) values (
    'https://repo4.url',
    current_timestamp - '6 months'::interval,
    '00000000-0004-0000-0000-000000000000'
);

-- Project 5
insert into project (
    project_id,
    name,
    foundation_id
) values (
    '00000000-0005-0000-0000-000000000000',
    'project5',
    'cncf'
);
insert into repository (
    repository_id,
    name,
    url,
    check_sets,
    project_id
) values (
    '00000000-0000-0005-0000-000000000000',
    'repository5',
    'https://repo5.url',
    '{code,community}',
    '00000000-0005-0000-0000-000000000000'
);
insert into report (
    report_id,
    data,
    repository_id
) values (
    '00000000-0000-0000-0005-000000000000',
    '{
        "documentation": {
            "annual_review": {
                "passed": false,
                "exempt": false,
                "failed": false
            }
        }
    }',
    '00000000-0000-0005-0000-000000000000'
);
insert into annual_review_notification (
    repository_url,
    issue_number,
    created_at,
    project_id
) values (
    'https://repo5.url',
    1,
    current_timestamp - '6 months'::interval,
    '00000000-0005-0000-0000-000000000000'
);
insert into annual_review_notification (
    repository_url,
    issue_number,
    created_at,
    project_id
) values (
    'https://repo5.url',
    2,
    current_timestamp - '3 months'::interval,
    '00000000-0005-0000-0000-000000000000'
);

-- Project 6
insert into project (
    project_id,
    name,
    foundation_id
) values (
    '00000000-0006-0000-0000-000000000000',
    'project6',
    'cncf'
);
insert into repository (
    repository_id,
    name,
    url,
    check_sets,
    project_id
) values (
    '00000000-0000-0006-0000-000000000000',
    'repository6',
    'https://repo6.url',
    '{code,community}',
    '00000000-0006-0000-0000-000000000000'
);
insert into report (
    report_id,
    data,
    repository_id
) values (
    '00000000-0000-0000-0006-000000000000',
    '{
        "documentation": {
            "annual_review": {
                "passed": false,
                "exempt": false,
                "failed": false
            }
        }
    }',
    '00000000-0000-0006-0000-000000000000'
);
insert into annual_review_notification (
    repository_url,
    issue_number,
    created_at,
    project_id
) values (
    'https://repo5.url',
    1,
    current_timestamp - '2 years'::interval,
    '00000000-0006-0000-0000-000000000000'
);

-- Run some tests
select results_eq(
    $$
        select * from get_pending_annual_review_notifications()
    $$,
    $$
        values
            ('00000000-0001-0000-0000-000000000000'::uuid, 'https://repo1.url', null::bigint),
            ('00000000-0004-0000-0000-000000000000'::uuid, 'https://repo4.url', null::bigint),
            ('00000000-0005-0000-0000-000000000000'::uuid, 'https://repo5.url', 2),
            ('00000000-0006-0000-0000-000000000000'::uuid, 'https://repo6.url', null::bigint)
    $$,
    'Return pending annual review notifications'
);

-- Finish tests and rollback transaction
select * from finish();
rollback;
