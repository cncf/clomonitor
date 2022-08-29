-- Returns all repositories including checks details.
create or replace function get_repositories_with_checks()
returns setof text as $$
    with repositories as (
        select
            p.foundation_id as foundation,
            p.name as project,
            r.url as repository_url,
            r.check_sets,
            (rp.data->'documentation'->'adopters'->'passed')::boolean as adopters,
            (rp.data->'documentation'->'changelog'->'passed')::boolean as changelog,
            (rp.data->'documentation'->'code_of_conduct'->'passed')::boolean as code_of_conduct,
            (rp.data->'documentation'->'contributing'->'passed')::boolean as contributing,
            (rp.data->'documentation'->'governance'->'passed')::boolean as governance,
            (rp.data->'documentation'->'maintainers'->'passed')::boolean as maintainers,
            (rp.data->'documentation'->'readme'->'passed')::boolean as readme,
            (rp.data->'documentation'->'roadmap'->'passed')::boolean as roadmap,
            (rp.data->'documentation'->'website'->'passed')::boolean as website,
            (rp.data->'license'->'license_approved'->'passed')::boolean as license_approved,
            (rp.data->'license'->'license_scanning'->'passed')::boolean as license_scanning,
            coalesce((rp.data->'license'->'license_spdx_id'->>'value')::text, 'Not detected') as license_spdx_id,
            (
                select string_agg(item #>> '{}', ' ')
                from jsonb_array_elements(rp.data->'best_practices'->'analytics'->'value') as item
            ) as analytics,
            (rp.data->'best_practices'->'artifacthub_badge'->'passed')::boolean as artifacthub_badge,
            (rp.data->'best_practices'->'cla'->'passed')::boolean as cla,
            (rp.data->'best_practices'->'community_meeting'->'passed')::boolean as community_meeting,
            (rp.data->'best_practices'->'dco'->'passed')::boolean as dco,
            (rp.data->'best_practices'->'github_discussions'->'passed')::boolean as github_discussions,
            (rp.data->'best_practices'->'openssf_badge'->'passed')::boolean as openssf_badge,
            (rp.data->'best_practices'->'recent_release'->'passed')::boolean as recent_release,
            (rp.data->'best_practices'->'slack_presence'->'passed')::boolean as slack_presence,
            (rp.data->'security'->'binary_artifacts'->'passed')::boolean as binary_artifacts,
            (rp.data->'security'->'code_review'->'passed')::boolean as code_review,
            (rp.data->'security'->'dangerous_workflow'->'passed')::boolean as dangerous_workflow,
            (rp.data->'security'->'dependency_update_tool'->'passed')::boolean as dependency_update_tool,
            (rp.data->'security'->'maintained'->'passed')::boolean as maintained,
            (rp.data->'security'->'sbom'->'passed')::boolean as sbom,
            (rp.data->'security'->'security_policy'->'passed')::boolean as security_policy,
            (rp.data->'security'->'signed_releases'->'passed')::boolean as signed_releases,
            (rp.data->'security'->'token_permissions'->'passed')::boolean as token_permissions,
            (rp.data->'legal'->'trademark_disclaimer'->'passed')::boolean as trademark_disclaimer
        from project p
        join repository r using (project_id)
        join report rp using (repository_id)
        order by p.foundation_id asc, p.name asc
    )
    select 'Foundation,Project,Repository URL,Check Sets,Adopters,Changelog,Code of Conduct,Contributing,Governance,Maintainers,Readme,Roadmap,Website,License Approved,License Scanning,License SPDX ID,Analytics,ArtifactHub Badge,CLA,Community Meeting,DCO,GitHub discussions,OpenSSF Badge,Recent Release,Slack Presence,Binary Artifacts,Code Review,Dangerous Workflow,Dependency Update Tool,Maintained,SBOM,Security Policy,Signed Releases,Token Permissions,Trademark Disclaimer'
    union all
    select rtrim(ltrim(r.*::text, '('), ')') from repositories r;
$$ language sql;
