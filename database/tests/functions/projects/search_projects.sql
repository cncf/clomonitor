-- Start transaction and plan tests
begin;
select plan(4);

-- No projects yet
select results_eq(
    $$
        select projects::jsonb, total_count::integer from search_projects('{}')
    $$,
    $$
        values ('[]'::jsonb, 0)
    $$,
    'No projects in database, an empty json array is returned'
);

-- Seed some data
insert into organization (
    organization_id,
    name,
    logo_url,
    foundation
) values (
    '00000001-0000-0000-0000-000000000000',
    'artifact-hub',
    'https://raw.githubusercontent.com/cncf/artwork/master/projects/artifacthub/icon/color/artifacthub-icon-color.svg',
    'cncf'
);
insert into organization (
    organization_id,
    name,
    logo_url,
    foundation
) values (
    '00000002-0000-0000-0000-000000000000',
    'containerd',
    'https://raw.githubusercontent.com/cncf/artwork/master/projects/containerd/icon/color/containerd-icon-color.svg',
    'cncf'
);
insert into organization (
    organization_id,
    name,
    logo_url,
    foundation
) values (
    '00000015-0000-0000-0000-000000000000',
    'tuf',
    'https://raw.githubusercontent.com/cncf/artwork/master/projects/tuf/icon/color/tuf-icon-color.svg',
    'cncf'
);
insert into project (
    project_id,
    name,
    display_name,
    description,
    category,
    home_url,
    devstats_url,
    score,
    rating,
    accepted_at,
    updated_at,
    maturity,
    organization_id
) values (
    '00000000-0001-0000-0000-000000000000',
    'artifact-hub',
    'Artifact Hub',
    'Artifact Hub is a web-based application that enables finding, installing, and publishing packages and configurations for CNCF projects.',
    'category1',
    'https://artifacthub.io',
    'https://artifacthub.devstats.cncf.io/',
    '{"k": "v"}',
    'a',
    '2020-01-01',
    '2022-02-25 12:54:17.80674+01',
    'sandbox',
    '00000001-0000-0000-0000-000000000000'
);
insert into project (
    project_id,
    name,
    description,
    category,
    home_url,
    devstats_url,
    score,
    rating,
    accepted_at,
    updated_at,
    organization_id,
    maturity
) values (
    '00000000-0002-0000-0000-000000000000',
    'containerd',
    'An industry-standard container runtime with an emphasis on simplicity, robustness and portability.',
    'category1',
    'https://containerd.io',
    'https://containerd.devstats.cncf.io',
    '{"k": "v"}',
    'a',
    '2021-01-01',
    '2022-02-25 12:54:25.952208+01',
    '00000002-0000-0000-0000-000000000000',
    'graduated'
);
insert into project (
    project_id,
    name,
    display_name,
    description,
    category,
    home_url,
    devstats_url,
    score,
    rating,
    accepted_at,
    updated_at,
    organization_id,
    maturity
) values (
    '00000000-0015-0000-0000-000000000000',
    'tuf',
    'The Update Framework',
    'Python reference implementation of The Update Framework (TUF).',
    'category2',
    'https://theupdateframework.com',
    'https://tuf.devstats.cncf.io',
    '{"k": "v"}',
    'b',
    '2022-01-01',
    '2022-02-25 12:54:23.937134+01',
    '00000015-0000-0000-0000-000000000000',
    'graduated'
);
insert into repository (
    repository_id,
    name,
    url,
    check_sets,
    project_id
) values (
    '00000000-0000-0001-0000-000000000000',
    'artifact-hub',
    'https://github.com/artifacthub/hub',
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
    'containerd',
    'https://github.com/containerd/containerd',
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
    '00000000-0000-0028-0000-000000000000',
    'python-tuf',
    'https://github.com/theupdateframework/python-tuf',
    '{code,community}',
    '00000000-0015-0000-0000-000000000000'
);
insert into repository (
    repository_id,
    name,
    url,
    check_sets,
    project_id
) values (
    '00000000-0000-0027-0000-000000000000',
    'tuf',
    'https://github.com/theupdateframework/specification',
    '{docs}',
    '00000000-0015-0000-0000-000000000000'
);

-- Run some tests

-- No filters
select results_eq(
    $$
        select projects::jsonb, total_count::integer from search_projects('{}')
    $$,
    $$
        values (
            '[
                {
                    "category": "category1",
                    "description": "Artifact Hub is a web-based application that enables finding, installing, and publishing packages and configurations for CNCF projects.",
                    "devstats_url": "https://artifacthub.devstats.cncf.io/",
                    "display_name": "Artifact Hub",
                    "id": "00000000-0001-0000-0000-000000000000",
                    "home_url": "https://artifacthub.io",
                    "logo_url": "https://raw.githubusercontent.com/cncf/artwork/master/projects/artifacthub/icon/color/artifacthub-icon-color.svg",
                    "maturity": "sandbox",
                    "name": "artifact-hub",
                    "organization": {
                        "name": "artifact-hub"
                    },
                    "rating": "a",
                    "repositories": [
                        {
                            "check_sets": ["code", "community"],
                            "name": "artifact-hub",
                            "url": "https://github.com/artifacthub/hub"
                        }
                    ],
                    "score": {"k": "v"},
                    "accepted_at": 1577836800,
                    "updated_at": 1645790057,
                    "foundation": "cncf"
                },
                {
                    "category": "category1",
                    "description": "An industry-standard container runtime with an emphasis on simplicity, robustness and portability.",
                    "devstats_url": "https://containerd.devstats.cncf.io",
                    "id": "00000000-0002-0000-0000-000000000000",
                    "home_url": "https://containerd.io",
                    "logo_url": "https://raw.githubusercontent.com/cncf/artwork/master/projects/containerd/icon/color/containerd-icon-color.svg",
                    "maturity": "graduated",
                    "name": "containerd",
                    "organization": {
                        "name": "containerd"
                    },
                    "rating": "a",
                    "repositories": [
                        {
                            "check_sets": ["code", "community"],
                            "name": "containerd",
                            "url": "https://github.com/containerd/containerd"
                        }
                    ],
                    "score": {"k": "v"},
                    "accepted_at": 1609459200,
                    "updated_at": 1645790065,
                    "foundation": "cncf"
                },
                {
                    "category": "category2",
                    "description": "Python reference implementation of The Update Framework (TUF).",
                    "devstats_url": "https://tuf.devstats.cncf.io",
                    "display_name": "The Update Framework",
                    "id": "00000000-0015-0000-0000-000000000000",
                    "home_url": "https://theupdateframework.com",
                    "logo_url": "https://raw.githubusercontent.com/cncf/artwork/master/projects/tuf/icon/color/tuf-icon-color.svg",
                    "maturity": "graduated",
                    "name": "tuf",
                    "organization": {
                        "name": "tuf"
                    },
                    "rating": "b",
                    "repositories": [
                        {
                            "check_sets": ["code", "community"],
                            "name": "python-tuf",
                            "url": "https://github.com/theupdateframework/python-tuf"
                        },
                        {
                            "check_sets": ["docs"],
                            "name": "tuf",
                            "url": "https://github.com/theupdateframework/specification"
                        }
                    ],
                    "score": {"k": "v"},
                    "accepted_at": 1640995200,
                    "updated_at": 1645790063,
                    "foundation": "cncf"
                }
            ]'::jsonb,
            3)
    $$,
    'Search projects with no filters'
);

-- Text filter
select results_eq(
    $$
        select projects::jsonb, total_count::integer from search_projects('{"text": "hub"}')
    $$,
    $$
        values (
            '[
                {
                    "category": "category1",
                    "description": "Artifact Hub is a web-based application that enables finding, installing, and publishing packages and configurations for CNCF projects.",
                    "devstats_url": "https://artifacthub.devstats.cncf.io/",
                    "display_name": "Artifact Hub",
                    "id": "00000000-0001-0000-0000-000000000000",
                    "home_url": "https://artifacthub.io",
                    "logo_url": "https://raw.githubusercontent.com/cncf/artwork/master/projects/artifacthub/icon/color/artifacthub-icon-color.svg",
                    "maturity": "sandbox",
                    "name": "artifact-hub",
                    "organization": {
                        "name": "artifact-hub"
                    },
                    "rating": "a",
                    "repositories": [
                        {
                            "check_sets": ["code", "community"],
                            "name": "artifact-hub",
                            "url": "https://github.com/artifacthub/hub"
                        }
                    ],
                    "score": {"k": "v"},
                    "accepted_at": 1577836800,
                    "updated_at": 1645790057,
                    "foundation": "cncf"
                }
            ]'::jsonb,
            1)
    $$,
    'Search projects with a text filter'
);

-- Text filter
select results_eq(
    $$
        select projects::jsonb, total_count::integer from search_projects('{"accepted_to": "2020-02-02"}')
    $$,
    $$
        values (
            '[
                {
                    "category": "category1",
                    "description": "Artifact Hub is a web-based application that enables finding, installing, and publishing packages and configurations for CNCF projects.",
                    "devstats_url": "https://artifacthub.devstats.cncf.io/",
                    "display_name": "Artifact Hub",
                    "id": "00000000-0001-0000-0000-000000000000",
                    "home_url": "https://artifacthub.io",
                    "logo_url": "https://raw.githubusercontent.com/cncf/artwork/master/projects/artifacthub/icon/color/artifacthub-icon-color.svg",
                    "maturity": "sandbox",
                    "name": "artifact-hub",
                    "organization": {
                        "name": "artifact-hub"
                    },
                    "rating": "a",
                    "repositories": [
                        {
                            "check_sets": ["code", "community"],
                            "name": "artifact-hub",
                            "url": "https://github.com/artifacthub/hub"
                        }
                    ],
                    "score": {"k": "v"},
                    "accepted_at": 1577836800,
                    "updated_at": 1645790057,
                    "foundation": "cncf"
                }
            ]'::jsonb,
            1)
    $$,
    'Search projects with an accepted date filter'
);

-- Finish tests and rollback transaction
select * from finish();
rollback;
