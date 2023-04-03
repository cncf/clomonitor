alter table foundation add column landscape_url text check (landscape_url <> '');

---- create above / drop below ----

alter table foundation drop column landscape_url;
