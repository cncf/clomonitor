alter table project
    alter column maturity type text using maturity::text,
    add constraint maturity_not_empty_check check (maturity <> '');;
drop type if exists maturity;

---- create above / drop below ----

create type maturity as enum ('graduated', 'incubating', 'sandbox');
alter table project
    drop constraint maturity_not_empty_check,
    alter column maturity type maturity using maturity::maturity;
