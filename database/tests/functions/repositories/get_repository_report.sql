-- Start transaction and plan tests
begin;
select plan(2);

-- Non existing repository
select is(
    get_repository_report('non-existing', 'non-existing', 'non-existing')::jsonb,
    (null::jsonb),
    'Null is returned if the requested repository does not exist'
);

-- Seed some data
insert into foundation values ('cncf', 'CNCF', 'http://127.0.0.1:8080/cncf.yaml');
insert into project (
    project_id,
    name,
    display_name,
    description,
    category,
    home_url,
    logo_url,
    devstats_url,
    score,
    rating,
    accepted_at,
    updated_at,
    maturity,
    foundation_id
) values (
    '00000000-0001-0000-0000-000000000000',
    'artifact-hub',
    'Artifact Hub',
    'Artifact Hub is a web-based application that enables finding, installing, and publishing packages and configurations for CNCF projects.',
    'category1',
    'https://artifacthub.io',
    'https://raw.githubusercontent.com/cncf/artwork/master/projects/artifacthub/icon/color/artifacthub-icon-color.svg',
    'https://artifacthub.devstats.cncf.io/',
    '{"k": "v"}',
    'a',
    '2021-01-01',
    '2022-02-24 09:40:42.695654+01',
    'sandbox',
    'cncf'
);
insert into repository (
    repository_id,
    name,
    url,
    check_sets,
    digest,
    score,
    project_id
) values (
    '00000000-0000-0001-0000-000000000000',
    'artifact-hub',
    'https://github.com/artifacthub/hub',
    '{code, community}',
    '653b5219d16a2e5be274a7fb765916789ae68fbb',
    '{"k": "v"}',
    '00000000-0001-0000-0000-000000000000'
);
insert into report (
    report_id,
    check_sets,
    data,
    updated_at,
    repository_id
) values (
    '5133b909-a5b3-4c24-87b1-16b02a955ffa',
    '{code, community}',
    '{"k": "v"}',
    '2022-02-24 09:40:42.695654+01',
    '00000000-0000-0001-0000-000000000000'
);

-- Run some tests
select is(
    get_repository_report('cncf', 'artifact-hub', 'artifact-hub')::jsonb,
    '{
        "name": "artifact-hub",
        "url": "https://github.com/artifacthub/hub",
        "check_sets": ["code", "community"],
        "score": {"k": "v"},
        "report": {"k": "v"}
    }'::jsonb,
    'Repository report returned as a json object'
);

-- Finish tests and rollback transaction
select * from finish();
rollback;
