alter table repository alter check_sets drop not null;

---- create above / drop below ----

alter table repository alter column check_sets set not null;
