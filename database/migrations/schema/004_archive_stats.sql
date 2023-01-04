create table if not exists stats_snapshot (
    foundation_id text references foundation on delete cascade,
    date date not null default current_date,
    data jsonb,
    unique (foundation_id, date)
);

---- create above / drop below ----

drop table if exists stats_snapshot;
