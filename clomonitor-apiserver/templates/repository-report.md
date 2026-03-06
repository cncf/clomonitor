## CLOMonitor report

### Summary

**Repository**: {{ name }}
**URL**: {{ url }}

{%- if let (Some(report), Some(score)) = (report.as_ref(), score.as_ref()) %}
**Checks sets**:  {% for check_set in check_sets %}`{{ check_set }}`{% if !loop.last %} + {% endif %}{% endfor %}
**Score**: {{ score.global.round() }}

### Checks passed per category

| Category       |                                           Score |
| :------------- | ----------------------------------------------: |
| Documentation  |  {{ category_score(score.documentation) }} |
| License        |        {{ category_score(score.license) }} |
| Best Practices | {{ category_score(score.best_practices) }} |
| Security       |       {{ category_score(score.security) }} |
| Legal          |          {{ category_score(score.legal) }} |

## Checks

{% if let Some(value) = score.documentation -%}
### Documentation [{{ value.round() }}%]

  {{ check("adopters", "Adopters", report.documentation.adopters) -}}
  {{ check("changelog", "Changelog", report.documentation.changelog) -}}
  {{ check("code-of-conduct", "Code of conduct", report.documentation.code_of_conduct) -}}
  {{ check("contributing", "Contributing", report.documentation.contributing) -}}
  {{ check("governance", "Governance", report.documentation.governance) -}}
  {{ check("maintainers", "Maintainers", report.documentation.maintainers) -}}
  {{ check("readme", "Readme", report.documentation.readme) -}}
  {{ check("roadmap", "Roadmap", report.documentation.roadmap) -}}
  {{ check("summary-table", "Summary Table", report.documentation.summary_table) -}}
  {{ check("website", "Website", report.documentation.website) -}}

{%- endif %}
{%- if let Some(value) = score.license %}
### License [{{ value.round() }}%]

  {{ license_spdx_id_check(report.license.license_spdx_id) -}}
  {{ check("approved-license", "Approved license", report.license.license_approved) -}}
  {{ check("license-scanning", "License scanning", report.license.license_scanning) -}}

{%- endif %}
{%- if let Some(value) = score.best_practices %}
### Best Practices [{{ value.round() }}%]

  {{ check("analytics", "Analytics", report.best_practices.analytics) -}}
  {{ check("artifact-hub-badge", "Artifact Hub badge", report.best_practices.artifacthub_badge) -}}
  {{ check("contributor-license-agreement", "Contributor License Agreement", report.best_practices.cla) -}}
  {{ check("community-meeting", "Community meeting", report.best_practices.community_meeting) -}}
  {{ check("developer-certificate-of-origin", "Developer Certificate of Origin", report.best_practices.dco) -}}
  {{ check("github-discussions", "Github discussions", report.best_practices.github_discussions) -}}
  {{ check("openssf-badge", "OpenSSF best practices badge", report.best_practices.openssf_badge) -}}
  {{ check("openssf-scorecard-badge", "OpenSSF Scorecard badge", report.best_practices.openssf_scorecard_badge) -}}
  {{ check("recent-release", "Recent release", report.best_practices.recent_release) -}}
  {{ check("slack-presence", "Slack precense", report.best_practices.slack_presence) -}}

{%- endif %}
{%- if let Some(value) = score.security %}
### Security [{{ value.round() }}%]

  {{ check("binary-artifacts-from-openssf-scorecard", "Binary artifacts", report.security.binary_artifacts) -}}
  {{ check("code-review-from-openssf-scorecard", "Code review", report.security.code_review) -}}
  {{ check("dangerous-workflow-from-openssf-scorecard", "Dangerous workflow", report.security.dangerous_workflow) -}}
  {{ check("dependencies-policy", "Dependencies policy", report.security.dependencies_policy) -}}
  {{ check("dependency-update-tool-from-openssf-scorecard", "Dependency update tool", report.security.dependency_update_tool) -}}
  {{ check("maintained-from-openssf-scorecard", "Maintained", report.security.maintained) -}}
  {{ check("software-bill-of-materials-sbom", "Software bill of materials (SBOM)", report.security.sbom) -}}
  {{ check("security-insights", "Security insights", report.security.security_insights) -}}
  {{ check("security-policy", "Security policy", report.security.security_policy) -}}
  {{ check("signed-releases-from-openssf-scorecard", "Signed releases", report.security.signed_releases) -}}
  {{ check("token-permissions-from-openssf-scorecard", "Token permissions", report.security.token_permissions) -}}

{%- endif %}
{%- if let Some(value) = score.legal %}
### Legal [{{ value.round() }}%]

  {{ check("trademark-disclaimer", "Trademark disclaimer", report.legal.trademark_disclaimer) -}}

{%- endif %}
For more information about the checks sets available and how each of the checks work, please see the [CLOMonitor's documentation](https://clomonitor.io/docs/topics/checks/).

{%- else %}

This repository hasn't been processed yet, please try again later.
{%- endif -%}

{% macro check(doc_id, display_name, option) %}
  {%- if let Some(check_output) = option -%}
    - [{% if check_output.passed || check_output.exempt %}x{% else %} {% endif %}]
    {%- if let Some(link) = check_output.url %} [{{ display_name }}]({{ link }}) {% else %} {{ display_name }} {% endif -%}
    ([_docs_](https://clomonitor.io/docs/topics/checks/#{{ doc_id }}))
    {%- if check_output.exempt %} `EXEMPT`{%- endif %}
    {%- if check_output.failed %} `CHECK FAILED`{%- endif %}
  {% endif -%}
{%- endmacro %}

{% macro license_spdx_id_check(option) %}
  {%- if let Some(check_output) = option -%}
    - [{% if check_output.passed || check_output.exempt %}x{% else %} {% endif %}] {{ check_output.value.as_deref().unwrap_or("Not detected") }} ([_docs_](https://clomonitor.io/docs/topics/checks/#spdx-id))
    {%- if check_output.exempt %} `EXEMPT`{%- endif %}
    {%- if check_output.failed %} `CHECK FAILED`{%- endif %}
  {% endif -%}
{%- endmacro %}

{% macro category_score(option) %}
  {%- if let Some(value) = option -%}{{ value.round() }}%{%- else -%}n/a{%- endif -%}
{% endmacro %}
