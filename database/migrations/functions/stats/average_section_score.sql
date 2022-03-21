-- Helper function that returns the average score of the provided section for
-- the reports of a given maturity (or all maturities when none is provided).
create or replace function average_section_score(p_section text, p_maturity_id integer)
returns real as $$
    select round(avg((score->>p_section)::real))
    from project
    where
        case when p_maturity_id is not null then
            maturity_id = p_maturity_id
        else true end;
$$ language sql;
