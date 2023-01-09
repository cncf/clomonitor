alter table project add column logo_dark_url text check (logo_dark_url <> '');

---- create above / drop below ----

alter table project drop column logo_dark_url;
