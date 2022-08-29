-- Helper function that returns the average score of the provided section for
-- the reports of a given maturity (or all maturities when none is provided).
create or replace function average_section_score(
    p_foundation text,
    p_section text,
    p_maturity text
)
returns real as $$
    select round(avg((p.score->>p_section)::real))::real
    from project p
    where p.score ? p_section
    and
        case when p_foundation is not null then
        p.foundation_id = p_foundation else true end
    and
        case when p_maturity is not null then
            p.maturity::text = p_maturity
        else true end;
$$ language sql;
