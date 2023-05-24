create table if not exists annual_review_notification (
    annual_review_notification_id uuid primary key default gen_random_uuid(),
    repository_url text not null,
    issue_number bigint,
    comment_id bigint,
    created_at timestamptz default current_timestamp not null,
    project_id uuid references project on delete cascade
);

---- create above / drop below ----

drop table if exists annual_review_notification;
