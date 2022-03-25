-- Start transaction and plan tests
begin;
select plan(1);

-- Seed some data
insert into organization (
    organization_id,
    name
) values (
    '00000001-0000-0000-0000-000000000000',
    'org1'
);
insert into project (
    project_id,
    name,
    score,
    rating,
    accepted_at,
    organization_id,
    category_id,
    maturity_id
) values (
    '00000000-0001-0000-0000-000000000000',
    'project1',
    '{"global": 95, "license": 100, "security": 100, "score_kind": "Primary", "documentation": 80, "best_practices": 100}',
    'a',
    '2022-02-25',
    '00000001-0000-0000-0000-000000000000',
    0,
    2
);
insert into project (
    project_id,
    name,
    score,
    rating,
    accepted_at,
    organization_id,
    category_id,
    maturity_id
) values (
    '00000000-0002-0000-0000-000000000000',
    'project2',
    '{"global": 70, "license": 80, "security": 60, "score_kind": "Primary", "documentation": 70, "best_practices": 70}',
    'b',
    '2021-02-24',
    '00000001-0000-0000-0000-000000000000',
    5,
    0
);
insert into project (
    project_id,
    name,
    score,
    rating,
    accepted_at,
    organization_id,
    category_id,
    maturity_id
) values (
    '00000000-0003-0000-0000-000000000000',
    'project3',
    '{"global": 55, "license": 50, "security": 60, "score_kind": "Primary", "documentation": 70, "best_practices": 40}',
    'c',
    '2021-02-25',
    '00000001-0000-0000-0000-000000000000',
    4,
    0
);
insert into repository (
    repository_id,
    name,
    url,
    kind,
    project_id
) values (
    '00000000-0000-0001-0000-000000000000',
    'repository1',
    'https://repo1.url',
    'primary',
    '00000000-0001-0000-0000-000000000000'
);
insert into repository (
    repository_id,
    name,
    url,
    kind,
    project_id
) values (
    '00000000-0000-0002-0000-000000000000',
    'repository2',
    'https://repo2.url',
    'primary',
    '00000000-0002-0000-0000-000000000000'
);
insert into repository (
    repository_id,
    name,
    url,
    kind,
    project_id
) values (
    '00000000-0000-0003-0000-000000000000',
    'repository3',
    'https://repo3.url',
    'primary',
    '00000000-0003-0000-0000-000000000000'
);
insert into report (
    report_id,
    data,
    updated_at,
    repository_id,
    linter_id
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
            }
        },
        "report_kind": "Primary",
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
            "dco": {
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
    '00000000-0000-0001-0000-000000000000',
    0
);
insert into report (
    report_id,
    data,
    updated_at,
    repository_id,
    linter_id
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
            }
        },
        "report_kind": "Primary",
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
            "dco": {
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
    '00000000-0000-0002-0000-000000000000',
    0
);
insert into report (
    report_id,
    data,
    updated_at,
    repository_id,
    linter_id
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
            }
        },
        "report_kind": "Primary",
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
            "dco": {
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
    '00000000-0000-0003-0000-000000000000',
    0
);

-- Run some tests
select is(
    get_stats()::jsonb - '{generated_at}'::text[],
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
                    "community_meeting": 0,
                    "dco": 67,
                    "openssf_badge": 67,
                    "recent_release": 67,
                    "slack_presence": 0
                },
                "security": {
                    "sbom": 0,
                    "security_policy": 67
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
