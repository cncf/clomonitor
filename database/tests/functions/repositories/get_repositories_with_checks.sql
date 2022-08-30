-- Start transaction and plan tests
begin;
select plan(1);

-- Seed some data
insert into project (
    project_id,
    name,
    category,
    score,
    rating,
    accepted_at,
    maturity,
    foundation_id
) values (
    '00000000-0001-0000-0000-000000000000',
    'project1',
    'category1',
    '{"global": 95.0, "license": 100.0, "security": 100.0, "documentation": 80.0, "best_practices": 100.0}',
    'a',
    '2022-02-25',
    'sandbox',
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
                "passed": true,
                "value": ["GA4"]
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
                "passed": false
            }
        }
    }',
    '2022-02-24 09:40:42.695654+01',
    '00000000-0000-0002-0000-000000000000'
);

-- Run some tests
select results_eq(
    $$
        select * from get_repositories_with_checks()
    $$,
    $$
        values
            ('Foundation,Project,Repository URL,Check Sets,Adopters,Changelog,Code of Conduct,Contributing,Governance,Maintainers,Readme,Roadmap,Website,License Approved,License Scanning,License SPDX ID,Analytics,ArtifactHub Badge,CLA,Community Meeting,DCO,GitHub discussions,OpenSSF Badge,Recent Release,Slack Presence,Binary Artifacts,Code Review,Dangerous Workflow,Dependency Update Tool,Maintained,SBOM,Security Policy,Signed Releases,Token Permissions,Trademark Disclaimer'),
            ('cncf,project1,https://repo1.url,"{code,community}",t,t,t,t,t,t,t,f,t,t,f,Apache-2.0,GA4,f,t,f,t,t,t,t,f,t,t,t,f,t,f,t,f,f,f'),
            ('cncf,project1,https://repo2.url,{docs},,,,,,,f,,,t,,Apache-2.0,,,,,,,,,,,,,,,,,,,')
    $$,
    'Return all repositories with all checks'
);

-- Finish tests and rollback transaction
select * from finish();
rollback;
