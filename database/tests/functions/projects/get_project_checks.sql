-- Start transaction and plan tests
begin;
select plan(1);

-- Seed some data
insert into organization (
    organization_id,
    name,
    foundation
) values (
    '00000001-0000-0000-0000-000000000000',
    'org1',
    'cncf'
);
insert into project (
    project_id,
    name,
    category,
    score,
    rating,
    accepted_at,
    maturity,
    organization_id
) values (
    '00000000-0001-0000-0000-000000000000',
    'project1',
    'category1',
    '{"global": 95.0, "license": 100.0, "security": 100.0, "documentation": 80.0, "best_practices": 100.0}',
    'a',
    '2022-02-25',
    'sandbox',
    '00000001-0000-0000-0000-000000000000'
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
    '{docs}',
    '00000000-0001-0000-0000-000000000000'
);
insert into report (
    report_id,
    data,
    updated_at,
    repository_id
) values (
    '00000000-0000-0000-0001-000000000000',
    '{
        "legal": {
            "trademark_disclaimer": {
                "passed": false
            }
        },
        "license": {
            "license_spdx_id": {
                "value": "Apache-2.0",
                "passed": true
            },
            "license_approved": {
                "value": true,
                "passed": true
            },
            "license_scanning": {
                "passed": false
            }
        },
        "security": {
            "sbom": {
                "passed": false
            },
            "security_policy": {
                "url": "https://github.com/fluent/fluentd/blob/master/SECURITY.md",
                "passed": true
            },
            "maintained": {
                "passed": true
            },
            "code_review": {
                "passed": true
            },
            "signed_releases": {
                "passed": false
            },
            "binary_artifacts": {
                "passed": true
            },
            "token_permissions": {
                "passed": false
            },
            "dangerous_workflow": {
                "passed": true
            },
            "dependency_update_tool": {
                "passed": false
            }
        },
        "documentation": {
            "readme": {
                "url": "https://github.com/fluent/fluentd/blob/master/README.md",
                "passed": true
            },
            "roadmap": {
                "passed": false
            },
            "website": {
                "url": "https://www.fluentd.org",
                "passed": true
            },
            "adopters": {
                "url": "https://github.com/fluent/fluentd/blob/master/ADOPTERS.md",
                "passed": true
            },
            "changelog": {
                "url": "https://github.com/fluent/fluentd/blob/master/CHANGELOG.md",
                "passed": true
            },
            "governance": {
                "url": "https://github.com/fluent/fluentd/blob/master/GOVERNANCE.md",
                "passed": true
            },
            "maintainers": {
                "url": "https://github.com/fluent/fluentd/blob/master/MAINTAINERS.md",
                "passed": true
            },
            "contributing": {
                "url": "https://github.com/fluent/fluentd/blob/master/CONTRIBUTING.md",
                "passed": true
            },
            "code_of_conduct": {
                "url": "https://github.com/fluent/fluentd/blob/master/code-of-conduct.md",
                "passed": true
            }
        },
        "best_practices": {
            "cla": {
                "passed": true
            },
            "dco": {
                "passed": true
            },
            "analytics": {
                "passed": true
            },
            "github_discussions": {
                "passed": true
            },
            "openssf_badge": {
                "url": "https://bestpractices.coreinfrastructure.org/projects/1189",
                "passed": true
            },
            "recent_release": {
                "url": "https://github.com/fluent/fluentd/releases/tag/v1.14.5",
                "passed": true
            },
            "slack_presence": {
                "passed": false
            },
            "artifacthub_badge": {
                "passed": false
            },
            "community_meeting": {
                "passed": false
            }
        }
    }',
    '2022-02-24 09:40:42.695654+01',
    '00000000-0000-0001-0000-000000000000'
);
insert into report (
    report_id,
    data,
    updated_at,
    repository_id
) values (
    '00000000-0000-0000-0002-000000000000',
    '{
        "license": {
            "license_spdx_id": {
                "value": "Apache-2.0",
                "passed": true
            },
            "license_approved": {
                "value": true,
                "passed": true
            }
        },
        "documentation": {
            "readme": {
                "url": "https://github.com/fluent/fluentd/blob/master/README.md",
                "passed": true
            }
        }
    }',
    '2022-02-24 09:40:42.695654+01',
    '00000000-0000-0002-0000-000000000000'
);

-- Run some tests
select results_eq(
    $$
        select * from get_project_checks('00000000-0001-0000-0000-000000000000')
    $$,
    $$
        values
            ('00000000-0000-0001-0000-000000000000'::uuid, 'legal', 'trademark_disclaimer', false),
            ('00000000-0000-0001-0000-000000000000'::uuid, 'license', 'license_spdx_id', true),
            ('00000000-0000-0001-0000-000000000000'::uuid, 'license', 'license_approved', true),
            ('00000000-0000-0001-0000-000000000000'::uuid, 'license', 'license_scanning', false),
            ('00000000-0000-0001-0000-000000000000'::uuid, 'security', 'sbom', false),
            ('00000000-0000-0001-0000-000000000000'::uuid, 'security', 'maintained', true),
            ('00000000-0000-0001-0000-000000000000'::uuid, 'security', 'code_review', true),
            ('00000000-0000-0001-0000-000000000000'::uuid, 'security', 'security_policy', true),
            ('00000000-0000-0001-0000-000000000000'::uuid, 'security', 'signed_releases', false),
            ('00000000-0000-0001-0000-000000000000'::uuid, 'security', 'binary_artifacts', true),
            ('00000000-0000-0001-0000-000000000000'::uuid, 'security', 'token_permissions', false),
            ('00000000-0000-0001-0000-000000000000'::uuid, 'security', 'dangerous_workflow', true),
            ('00000000-0000-0001-0000-000000000000'::uuid, 'security', 'dependency_update_tool', false),
            ('00000000-0000-0001-0000-000000000000'::uuid, 'documentation', 'readme', true),
            ('00000000-0000-0001-0000-000000000000'::uuid, 'documentation', 'roadmap', false),
            ('00000000-0000-0001-0000-000000000000'::uuid, 'documentation', 'website', true),
            ('00000000-0000-0001-0000-000000000000'::uuid, 'documentation', 'adopters', true),
            ('00000000-0000-0001-0000-000000000000'::uuid, 'documentation', 'changelog', true),
            ('00000000-0000-0001-0000-000000000000'::uuid, 'documentation', 'governance', true),
            ('00000000-0000-0001-0000-000000000000'::uuid, 'documentation', 'maintainers', true),
            ('00000000-0000-0001-0000-000000000000'::uuid, 'documentation', 'contributing', true),
            ('00000000-0000-0001-0000-000000000000'::uuid, 'documentation', 'code_of_conduct', true),
            ('00000000-0000-0001-0000-000000000000'::uuid, 'best_practices', 'cla', true),
            ('00000000-0000-0001-0000-000000000000'::uuid, 'best_practices', 'dco', true),
            ('00000000-0000-0001-0000-000000000000'::uuid, 'best_practices', 'analytics', true),
            ('00000000-0000-0001-0000-000000000000'::uuid, 'best_practices', 'openssf_badge', true),
            ('00000000-0000-0001-0000-000000000000'::uuid, 'best_practices', 'recent_release', true),
            ('00000000-0000-0001-0000-000000000000'::uuid, 'best_practices', 'slack_presence', false),
            ('00000000-0000-0001-0000-000000000000'::uuid, 'best_practices', 'artifacthub_badge', false),
            ('00000000-0000-0001-0000-000000000000'::uuid, 'best_practices', 'community_meeting', false),
            ('00000000-0000-0001-0000-000000000000'::uuid, 'best_practices', 'github_discussions', true),
            ('00000000-0000-0002-0000-000000000000'::uuid, 'license', 'license_spdx_id', true),
            ('00000000-0000-0002-0000-000000000000'::uuid, 'license', 'license_approved', true),
            ('00000000-0000-0002-0000-000000000000'::uuid, 'documentation', 'readme', true)
    $$,
    'Return all project checks, one row per check'
);

-- Finish tests and rollback transaction
select * from finish();
rollback;
