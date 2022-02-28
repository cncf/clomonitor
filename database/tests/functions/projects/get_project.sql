-- Start transaction and plan tests
begin;
select plan(2);

-- Non existing project
select is(
    get_project('non-existing', 'non-existing')::jsonb,
    (null::jsonb),
    'Null is returned if the requested project does not exist'
);

-- Seed some data
insert into organization (
    organization_id,
    name,
    logo_url
) values (
    '00000001-0000-0000-0000-000000000000',
    'artifact-hub',
    'https://raw.githubusercontent.com/cncf/artwork/master/projects/artifacthub/icon/color/artifacthub-icon-color.svg'
);
insert into project (
    project_id,
    name,
    display_name,
    description,
    home_url,
    devstats_url,
    score,
    rating,
    updated_at,
    organization_id,
    category_id,
    maturity_id
) values (
    '00000000-0001-0000-0000-000000000000',
    'artifact-hub',
    'Artifact Hub',
    'Artifact Hub is a web-based application that enables finding, installing, and publishing packages and configurations for CNCF projects.',
    'https://artifacthub.io',
    'https://artifacthub.devstats.cncf.io/',
    '{"global": 95, "license": 100, "security": 100, "score_kind": "Primary", "documentation": 80, "best_practices": 100}',
    'a',
    '2022-02-24 09:40:42.695654+01',
    '00000001-0000-0000-0000-000000000000',
    0,
    2
);
insert into repository (
    repository_id,
    name,
    url,
    kind,
    digest,
    score,
    project_id
) values (
    '00000000-0000-0001-0000-000000000000',
    'artifact-hub',
    'https://github.com/artifacthub/hub',
    'primary',
    '653b5219d16a2e5be274a7fb765916789ae68fbb',
    '{"global": 95, "license": 100, "security": 100, "score_kind": "Primary", "documentation": 80, "best_practices": 100}',
    '00000000-0001-0000-0000-000000000000'
);
insert into report (
    report_id,
    data,
    updated_at,
    repository_id,
    linter_id
) values (
    '5133b909-a5b3-4c24-87b1-16b02a955ffa',
    '{
        "license": {
            "spdx_id": "Apache-2.0",
            "approved": true,
            "scanning": "https://license-scanning.url"
        },
        "security": {
            "security_policy": true
        },
        "report_kind": "Primary",
        "documentation": {
            "readme": true,
            "roadmap": false,
            "website": true,
            "adopters": false,
            "changelog": true,
            "governance": false,
            "maintainers": true,
            "contributing": true,
            "code_of_conduct": true
        },
        "best_practices": {
            "openssf_badge": true,
            "recent_release": true,
            "artifacthub_badge": true,
            "community_meeting": true
        }
    }',
    '2022-02-24 09:40:42.695654+01',
    '00000000-0000-0001-0000-000000000000',
    0
);

-- Run some tests
select is(
    get_project('artifact-hub', 'artifact-hub')::jsonb,
    '{
        "category_id": 0,
        "description": "Artifact Hub is a web-based application that enables finding, installing, and publishing packages and configurations for CNCF projects.",
        "devstats_url": "https://artifacthub.devstats.cncf.io/",
        "display_name": "Artifact Hub",
        "home_url": "https://artifacthub.io",
        "id": "00000000-0001-0000-0000-000000000000",
        "logo_url": "https://raw.githubusercontent.com/cncf/artwork/master/projects/artifacthub/icon/color/artifacthub-icon-color.svg",
        "maturity_id": 2,
        "name": "artifact-hub",
        "rating": "a",
        "repositories": [
            {
                "digest": "653b5219d16a2e5be274a7fb765916789ae68fbb",
                "kind": "primary",
                "name": "artifact-hub",
                "reports": [
                    {
                        "data": {
                            "best_practices": {
                                "artifacthub_badge": true,
                                "community_meeting": true,
                                "openssf_badge": true,
                                "recent_release": true
                            },
                            "documentation": {
                                "adopters": false,
                                "changelog": true,
                                "code_of_conduct": true,
                                "contributing": true,
                                "governance": false,
                                "maintainers": true,
                                "readme": true,
                                "roadmap": false,
                                "website": true
                            },
                            "license": {
                                "approved": true,
                                "scanning": "https://license-scanning.url",
                                "spdx_id": "Apache-2.0"
                            },
                            "report_kind": "Primary",
                            "security": {
                                "security_policy": true
                            }
                        },
                        "errors": null,
                        "linter_id": 0,
                        "report_id": "5133b909-a5b3-4c24-87b1-16b02a955ffa",
                        "updated_at": 1645692042
                    }
                ],
                "repository_id": "00000000-0000-0001-0000-000000000000",
                "score": {
                    "best_practices": 100,
                    "documentation": 80,
                    "global": 95,
                    "license": 100,
                    "score_kind": "Primary",
                    "security": 100
                },
                "url": "https://github.com/artifacthub/hub"
            }
        ],
        "score": {
            "best_practices": 100,
            "documentation": 80,
            "global": 95,
            "license": 100,
            "score_kind": "Primary",
            "security": 100
        },
        "updated_at": 1645692042
    }'::jsonb,
    'Project returned as a json object'
);

-- Finish tests and rollback transaction
select * from finish();
rollback;
