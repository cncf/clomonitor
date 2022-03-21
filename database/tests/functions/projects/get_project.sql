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
