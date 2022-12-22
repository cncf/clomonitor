create table if not exists project_views (
     project_id uuid references project on delete set null,
     day date not null,
     total integer not null,
     unique (project_id, day)
 );

---- create above / drop below ----

drop table if exists project_views;
