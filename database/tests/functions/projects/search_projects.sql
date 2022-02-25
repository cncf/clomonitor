-- Start transaction and plan tests
begin;
select plan(3);

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
    logo_url
) values (
    '00000001-0000-0000-0000-000000000000',
    'artifact-hub',
    'https://raw.githubusercontent.com/cncf/artwork/master/projects/artifacthub/icon/color/artifacthub-icon-color.svg'
);
insert into organization (
    organization_id,
    name,
    logo_url
) values (
    '00000002-0000-0000-0000-000000000000',
    'containerd',
    'https://raw.githubusercontent.com/cncf/artwork/master/projects/containerd/icon/color/containerd-icon-color.svg'
);
insert into organization (
    organization_id,
    name,
    logo_url
) values (
    '00000015-0000-0000-0000-000000000000',
    'tuf',
    'https://raw.githubusercontent.com/cncf/artwork/master/projects/tuf/icon/color/tuf-icon-color.svg'
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
    '2022-02-25 12:54:17.80674+01',
    '00000001-0000-0000-0000-000000000000',
    0,
    2
);
insert into project (
    project_id,
    name,
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
    '00000000-0002-0000-0000-000000000000',
    'containerd',
    'An industry-standard container runtime with an emphasis on simplicity, robustness and portability.',
    'https://containerd.io',
    'https://containerd.devstats.cncf.io',
    '{"global": 80, "license": 80, "security": 100, "score_kind": "Primary", "documentation": 70, "best_practices": 70}',
    'a',
    '2022-02-25 12:54:25.952208+01',
    '00000002-0000-0000-0000-000000000000',
    5,
    0
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
    '00000000-0015-0000-0000-000000000000',
    'tuf',
    'The Update Framework',
    'Python reference implementation of The Update Framework (TUF).',
    'https://theupdateframework.com',
    'https://tuf.devstats.cncf.io',
    '{"global": 65, "license": 84, "security": 0, "score_kind": "Primary", "documentation": 84, "best_practices": 70}',
    'b',
    '2022-02-25 12:54:23.937134+01',
    '00000015-0000-0000-0000-000000000000',
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
    'artifact-hub',
    'https://github.com/artifacthub/hub',
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
    'containerd',
    'https://github.com/containerd/containerd',
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
    '00000000-0000-0028-0000-000000000000',
    'python-tuf',
    'https://github.com/theupdateframework/python-tuf',
    'primary',
    '00000000-0015-0000-0000-000000000000'
);
insert into repository (
    repository_id,
    name,
    url,
    kind,
    project_id
) values (
    '00000000-0000-0027-0000-000000000000',
    'tuf',
    'https://github.com/theupdateframework/specification',
    'secondary',
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
                    "category_id": 0,
                    "description": "Artifact Hub is a web-based application that enables finding, installing, and publishing packages and configurations for CNCF projects.",
                    "devstats_url": "https://artifacthub.devstats.cncf.io/",
                    "display_name": "Artifact Hub",
                    "id": "00000000-0001-0000-0000-000000000000",
                    "home_url": "https://artifacthub.io",
                    "logo_url": "https://raw.githubusercontent.com/cncf/artwork/master/projects/artifacthub/icon/color/artifacthub-icon-color.svg",
                    "maturity_id": 2,
                    "name": "artifact-hub",
                    "organization": {
                        "name": "artifact-hub"
                    },
                    "rating": "a",
                    "repositories": [
                        {
                            "kind": "primary",
                            "name": "artifact-hub",
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
                    "updated_at": 1645790057
                },
                {
                    "category_id": 5,
                    "description": "An industry-standard container runtime with an emphasis on simplicity, robustness and portability.",
                    "devstats_url": "https://containerd.devstats.cncf.io",
                    "id": "00000000-0002-0000-0000-000000000000",
                    "home_url": "https://containerd.io",
                    "logo_url": "https://raw.githubusercontent.com/cncf/artwork/master/projects/containerd/icon/color/containerd-icon-color.svg",
                    "maturity_id": 0,
                    "name": "containerd",
                    "organization": {
                        "name": "containerd"
                    },
                    "rating": "a",
                    "repositories": [
                        {
                            "kind": "primary",
                            "name": "containerd",
                            "url": "https://github.com/containerd/containerd"
                        }
                    ],
                    "score": {
                        "best_practices": 70,
                        "documentation": 70,
                        "global": 80,
                        "license": 80,
                        "score_kind": "Primary",
                        "security": 100
                    },
                    "updated_at": 1645790065
                },
                {
                    "category_id": 4,
                    "description": "Python reference implementation of The Update Framework (TUF).",
                    "devstats_url": "https://tuf.devstats.cncf.io",
                    "display_name": "The Update Framework",
                    "id": "00000000-0015-0000-0000-000000000000",
                    "home_url": "https://theupdateframework.com",
                    "logo_url": "https://raw.githubusercontent.com/cncf/artwork/master/projects/tuf/icon/color/tuf-icon-color.svg",
                    "maturity_id": 0,
                    "name": "tuf",
                    "organization": {
                        "name": "tuf"
                    },
                    "rating": "b",
                    "repositories": [
                        {
                            "kind": "primary",
                            "name": "python-tuf",
                            "url": "https://github.com/theupdateframework/python-tuf"
                        },
                        {
                            "kind": "secondary",
                            "name": "tuf",
                            "url": "https://github.com/theupdateframework/specification"
                        }
                    ],
                    "score": {
                        "best_practices": 70,
                        "documentation": 84,
                        "global": 65,
                        "license": 84,
                        "score_kind": "Primary",
                        "security": 0
                    },
                    "updated_at": 1645790063
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
                    "category_id": 0,
                    "description": "Artifact Hub is a web-based application that enables finding, installing, and publishing packages and configurations for CNCF projects.",
                    "devstats_url": "https://artifacthub.devstats.cncf.io/",
                    "display_name": "Artifact Hub",
                    "id": "00000000-0001-0000-0000-000000000000",
                    "home_url": "https://artifacthub.io",
                    "logo_url": "https://raw.githubusercontent.com/cncf/artwork/master/projects/artifacthub/icon/color/artifacthub-icon-color.svg",
                    "maturity_id": 2,
                    "name": "artifact-hub",
                    "organization": {
                        "name": "artifact-hub"
                    },
                    "rating": "a",
                    "repositories": [
                        {
                            "kind": "primary",
                            "name": "artifact-hub",
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
                    "updated_at": 1645790057
                }
            ]'::jsonb,
            1)
    $$,
    'Search projects with a text filter'
);

-- Finish tests and rollback transaction
select * from finish();
rollback;
