insert into organization (organization_id, name, home_url, logo_url)
values ('00000001-0000-0000-0000-000000000000', 'artifact-hub', 'https://artifacthub.io/', 'https://raw.githubusercontent.com/cncf/artwork/master/projects/artifacthub/icon/color/artifacthub-icon-color.svg');
insert into organization (organization_id, name, home_url, logo_url)
values ('00000002-0000-0000-0000-000000000000', 'containerd', 'https://containerd.io', 'https://raw.githubusercontent.com/cncf/artwork/master/projects/containerd/icon/color/containerd-icon-color.svg');
insert into organization (organization_id, name, home_url, logo_url)
values ('00000003-0000-0000-0000-000000000000', 'core-dns', 'https://coredns.io', 'https://raw.githubusercontent.com/cncf/artwork/master/projects/coredns/icon/color/coredns-icon-color.svg');

insert into project (project_id, maturity_id, category_id, name, display_name, description, devstats_url, organization_id)
values ('00000000-0001-0000-0000-000000000000', 2, 0, 'artifact-hub', 'Artifact Hub', 'Artifact Hub is a web-based application that enables finding, installing, and publishing packages and configurations for CNCF projects.', 'https://artifacthub.devstats.cncf.io/', '00000001-0000-0000-0000-000000000000');
insert into project (project_id, maturity_id, category_id, name, description, devstats_url, organization_id)
values ('00000000-0002-0000-0000-000000000000', 0, 5, 'containerd', 'An industry-standard container runtime with an emphasis on simplicity, robustness and portability.', 'https://containerd.devstats.cncf.io', '00000002-0000-0000-0000-000000000000');
insert into project (project_id, maturity_id, category_id, name, display_name, description, devstats_url, organization_id)
values ('00000000-0003-0000-0000-000000000000', 0, 2, 'core-dns', 'CoreDNS', 'CoreDNS is a DNS server. It is written in Go. It can be used in a multitude of environments because of its flexibility.', 'https://coredns.devstats.cncf.io', '00000003-0000-0000-0000-000000000000');

insert into repository (repository_id, name, url, kind, project_id)
values ('00000000-0000-0001-0000-000000000000', 'artifact-hub', 'https://github.com/artifacthub/hub', 'primary', '00000000-0001-0000-0000-000000000000');
insert into repository (repository_id, name, url, kind, project_id)
values ('00000000-0000-0002-0000-000000000000', 'containerd', 'https://github.com/containerd/containerd', 'primary', '00000000-0002-0000-0000-000000000000');
insert into repository (repository_id, name, url, kind, project_id)
values ('00000000-0000-0003-0000-000000000000', 'coredns', 'https://github.com/coredns/coredns', 'primary', '00000000-0003-0000-0000-000000000000');
