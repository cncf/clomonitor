insert into project (project_id, name, display_name, description, category, devstats_url, maturity, foundation_id)
values ('00000000-0001-0000-0000-000000000000', 'artifact-hub', 'Artifact Hub', 'Artifact Hub is a web-based application that enables finding, installing, and publishing packages and configurations for CNCF projects.', 'app definition', 'https://artifacthub.devstats.cncf.io/', 'sandbox', 'cncf');
insert into project (project_id, name, description, category, devstats_url, maturity, foundation_id)
values ('00000000-0002-0000-0000-000000000000', 'containerd', 'An industry-standard container runtime with an emphasis on simplicity, robustness and portability.', 'runtime', 'https://containerd.devstats.cncf.io', 'graduated', 'cncf');
insert into project (project_id, name, display_name, category, description, devstats_url, maturity, foundation_id)
values ('00000000-0003-0000-0000-000000000000', 'core-dns', 'CoreDNS', 'CoreDNS is a DNS server. It is written in Go. It can be used in a multitude of environments because of its flexibility.', 'orchestration', 'https://coredns.devstats.cncf.io', 'graduated', 'cncf');

insert into repository (repository_id, name, url, check_sets, project_id)
values ('00000000-0000-0001-0000-000000000000', 'artifact-hub', 'https://github.com/artifacthub/hub', '{community,code}', '00000000-0001-0000-0000-000000000000');
insert into repository (repository_id, name, url, check_sets, project_id)
values ('00000000-0000-0002-0000-000000000000', 'containerd', 'https://github.com/containerd/containerd', '{community,code}', '00000000-0002-0000-0000-000000000000');
insert into repository (repository_id, name, url, check_sets, project_id)
values ('00000000-0000-0003-0000-000000000000', 'coredns', 'https://github.com/coredns/coredns', '{community,code}', '00000000-0003-0000-0000-000000000000');
