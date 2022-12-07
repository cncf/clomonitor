create index project_foundation_id_idx on project (foundation_id);
create index repository_project_id_idx on repository (project_id);
create index report_repository_id_idx on report (repository_id);

---- create above / drop below ----

drop index project_foundation_id_idx;
drop index repository_project_id_idx;
drop index report_repository_id_idx;
