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
    '00000000-0002-0000-0000-000000000000',
    'project2',
    'category1',
    '{"global": 70.0, "license": 80.0, "security": 60.0, "documentation": 70.0, "best_practices": 70.0}',
    'b',
    '2021-02-24',
    'graduated',
    '00000001-0000-0000-0000-000000000000'
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
    '00000000-0003-0000-0000-000000000000',
    'project3',
    'category2',
    '{"global": 55.0, "license": 50.0, "security": 60.0, "documentation": 70.0, "best_practices": 40.0}',
    'c',
    '2021-02-25',
    'graduated',
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
    '{code,community}',
    '00000000-0002-0000-0000-000000000000'
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
            "spdx_id": {
                "value": "Apache-2.0",
                "passed": true
            },
            "approved": {
                "value": true,
                "passed": true
            },
            "scanning": {
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
            "vulnerabilities": {
                "passed": true
            },
            "binary_artifacts": {
                "passed": true
            },
            "branch_protection": {
                "passed": false
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
            "ga4": {
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
        "legal": {
            "trademark_disclaimer": {
                "passed": false
            }
        },
        "license": {
            "spdx_id": {
                "value": "Apache-2.0",
                "passed": true
            },
            "approved": {
                "value": true,
                "passed": true
            },
            "scanning": {
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
            "vulnerabilities": {
                "passed": true
            },
            "binary_artifacts": {
                "passed": true
            },
            "branch_protection": {
                "passed": false
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
            "ga4": {
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
    '00000000-0000-0002-0000-000000000000'
);
insert into report (
    report_id,
    data,
    updated_at,
    repository_id
) values (
    '00000000-0000-0000-0003-000000000000',
    '{
        "legal": {
            "trademark_disclaimer": {
                "passed": false
            }
        },
        "license": {
            "spdx_id": {
                "passed": false
            },
            "approved": {
                "passed": false
            },
            "scanning": {
                "passed": false
            }
        },
        "security": {
            "sbom": {
                "passed": false
            },
            "security_policy": {
                "passed": false
            },
            "maintained": {
                "passed": false
            },
            "code_review": {
                "passed": false
            },
            "signed_releases": {
                "passed": false
            },
            "vulnerabilities": {
                "passed": false
            },
            "binary_artifacts": {
                "passed": false
            },
            "branch_protection": {
                "passed": false
            },
            "token_permissions": {
                "passed": false
            },
            "dangerous_workflow": {
                "passed": false
            },
            "dependency_update_tool": {
                "passed": false
            }
        },
        "documentation": {
            "readme": {
                "passed": false
            },
            "roadmap": {
                "passed": false
            },
            "website": {
                "passed": false
            },
            "adopters": {
                "passed": false
            },
            "changelog": {
                "passed": false
            },
            "governance": {
                "passed": false
            },
            "maintainers": {
                "passed": false
            },
            "contributing": {
                "passed": false
            },
            "code_of_conduct": {
                "passed": false
            }
        },
        "best_practices": {
            "cla": {
                "passed": false
            },
            "dco": {
                "passed": false
            },
            "ga4": {
                "passed": false
            },
            "github_discussions": {
                "passed": false
            },
            "openssf_badge": {
                "passed": false
            },
            "recent_release": {
                "passed": false
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
    '00000000-0000-0003-0000-000000000000'
);

-- Run some tests
select is(
    get_stats('cncf')::jsonb - '{generated_at}'::text[],
    '{
        "projects": {
            "running_total": [
                [1612137600000, 2],
                [1643673600000, 3]
            ],
            "rating_distribution": {
                "all": [
                    {"a": 1},
                    {"b": 1},
                    {"c": 1}
                ],
                "graduated": [
                    {"b": 1},
                    {"c": 1}
                ],
                "sandbox": [
                    {"a": 1}
                ]
            },
            "accepted_distribution": [
                {"year": 2022, "month": 2, "total": 1},
                {"year": 2021, "month": 2, "total": 2}
            ],
            "sections_average": {
                "all": {
                    "license": 77,
                    "security": 73,
                    "documentation": 73,
                    "best_practices": 70
                },
                "sandbox": {
                    "license": 100,
                    "security": 100,
                    "documentation": 80,
                    "best_practices": 100
                },
                "graduated": {
                    "license": 65,
                    "security": 60,
                    "documentation": 70,
                    "best_practices": 55
                },
                "incubating": {
                }
            }
        },
        "repositories": {
            "passing_check": {
                "documentation": {
                    "adopters": 67,
                    "changelog": 67,
                    "code_of_conduct": 67,
                    "contributing": 67,
                    "governance": 67,
                    "maintainers": 67,
                    "readme": 67,
                    "roadmap": 0,
                    "website": 67
                },
                "license": {
                    "approved": 67,
                    "scanning": 0,
                    "spdx_id": 67
                },
                "best_practices": {
                    "artifacthub_badge": 0,
                    "cla": 67,
                    "community_meeting": 0,
                    "dco": 67,
                    "ga4": 67,
                    "github_discussions": 67,
                    "openssf_badge": 67,
                    "recent_release": 67,
                    "slack_presence": 0
                },
                "security": {
                    "binary_artifacts": 67,
                    "branch_protection": 0,
                    "code_review": 67,
                    "dangerous_workflow": 67,
                    "dependency_update_tool": 0,
                    "maintained": 67,
                    "sbom": 0,
                    "security_policy": 67,
                    "signed_releases": 0,
                    "token_permissions": 0,
                    "vulnerabilities": 67
                },
                "legal": {
                    "trademark_disclaimer": 0
                }
            }
        }
    }'::jsonb,
    'Stats returned as a json object'
);

-- Finish tests and rollback transaction
select * from finish();
rollback;
