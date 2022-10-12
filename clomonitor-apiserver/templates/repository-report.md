## CLOMonitor report

### Summary

**Repository**: {{ name }}
**URL**: {{ url }}

{%- if let (Some(report), Some(score)) = (report.as_ref(), score.as_ref()) %}
**Checks sets**:  {% for check_set in check_sets %}`{{ check_set }}`{% if !loop.last %} + {% endif %}{% endfor %}
**Score**: {{ score.global.round() }}

### Checks passed per category

| Category  |  Score  |
| :----------------- | --------: |
|  Documentation  |  {% call category_score(score.documentation) %}  |
|  License  |  {% call category_score(score.license) %}  |
|  Best Practices  |  {% call category_score(score.best_practices) %}  |
|  Security  |  {% call category_score(score.security) %}  |
|  Legal  |  {% call category_score(score.legal) %}  |

## Checks

{% if let Some(value) = score.documentation -%}
### Documentation [{{ value.round() }}%]

  {% call check("adopters", "Adopters", report.documentation.adopters) -%}
  {% call check("changelog", "Changelog", report.documentation.changelog) -%}
  {% call check("code_of_conduct", "Code of conduct", report.documentation.code_of_conduct) -%}
  {% call check("contributing", "Contributing", report.documentation.contributing) -%}
  {% call check("governance", "Governance", report.documentation.governance) -%}
  {% call check("maintainers", "Maintainers", report.documentation.maintainers) -%}
  {% call check("readme", "Readme", report.documentation.readme) -%}
  {% call check("roadmap", "Roadmap", report.documentation.roadmap) -%}
  {% call check("website", "Website", report.documentation.website) -%}

{%- endif %}
{%- if let Some(value) = score.license %}
### License [{{ value.round() }}%]

  {% call license_spdx_id_check("license_spdx_id", "SPDX", report.license.license_spdx_id) -%}
  {% call check("license_approved", "Approved license", report.license.license_approved) -%}
  {% call check("license_scanning", "License scanning", report.license.license_scanning) -%}

{%- endif %}
{%- if let Some(value) = score.best_practices %}
### Best Practices [{{ value.round() }}%]

  {% call check("analytics", "Analytics", report.best_practices.analytics) -%}
  {% call check("artifacthub_badge", "Artifact Hub badge", report.best_practices.artifacthub_badge) -%}
  {% call check("cla", "Contributor License Agreement", report.best_practices.cla) -%}
  {% call check("community_meeting", "Community meeting", report.best_practices.community_meeting) -%}
  {% call check("dco", "Developer Certificate of Origin", report.best_practices.dco) -%}
  {% call check("github_discussions", "Github discussions", report.best_practices.github_discussions) -%}
  {% call check("openssf_badge", "OpenSSF badge", report.best_practices.openssf_badge) -%}
  {% call check("recent_release", "Recent release", report.best_practices.recent_release) -%}
  {% call check("slack_presence", "Slack precense", report.best_practices.slack_presence) -%}

{%- endif %}
{%- if let Some(value) = score.security %}
### Security [{{ value.round() }}%]

  {% call check("binary_artifacts", "Binary artifacts", report.security.binary_artifacts) -%}
  {% call check("code_review", "Code review", report.security.code_review) -%}
  {% call check("dangerous_workflow", "Dangerous workflow", report.security.dangerous_workflow) -%}
  {% call check("dependency_update_tool", "Dependency update tool", report.security.dependency_update_tool) -%}
  {% call check("maintained", "Maintained", report.security.maintained) -%}
  {% call check("sbom", "Software bill of materials (SBOM)", report.security.sbom) -%}
  {% call check("security_policy", "Security policy", report.security.security_policy) -%}
  {% call check("signed_releases", "Signed releases", report.security.signed_releases) -%}
  {% call check("token_permissions", "Token permissions", report.security.token_permissions) -%}

{%- endif %}
{%- if let Some(value) = score.legal %}
### Legal [{{ value.round() }}%]

  {% call check("trademark_disclaimer", "Trademark disclaimer", report.legal.trademark_disclaimer) -%}

{%- endif %}
For more information about the checks sets available and how each of the checks work, please see the [CLOMonitor's documentation](https://clomonitor.io/docs/topics/checks/).

{%- else %}

This repository hasn't been processed yet, please try again later.
{%- endif -%}

{% macro check(name, display_name, option) %}
  {%- if let Some(check_output) = option -%}
    - [{% if check_output.passed || check_output.exempt %}x{% else %} {% endif %}]
    {%- if let Some(link) = check_output.url %} [{{ display_name }}]({{ link }}) {% else %} {{ display_name }} {% endif -%}
    ([_docs_](https://clomonitor.io/docs/topics/checks/#{{ name }}))
    {%- if check_output.exempt %} `EXEMPT`{%- endif %}
    {%- if check_output.failed %} `CHECK FAILED`{%- endif %}
  {% endif -%}
{%- endmacro %}

{% macro license_spdx_id_check(name, display_name, option) %}
  {%- if let Some(check_output) = option -%}
    - [{% if check_output.passed || check_output.exempt %}x{% else %} {% endif %}] {{ check_output.value.as_deref().unwrap_or("Not detected") }} ([_docs_](https://clomonitor.io/docs/topics/checks/#{{ name }}))
    {%- if check_output.exempt %} `EXEMPT`{%- endif %}
    {%- if check_output.failed %} `CHECK FAILED`{%- endif %}
  {% endif -%}
{%- endmacro %}

{% macro category_score(option) %}
  {%- if let Some(value) = option -%}{{ value.round() }}%{%- else -%}n/a{%- endif -%}
{% endmacro %}
