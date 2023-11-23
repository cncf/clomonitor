# Checks

**CLOMonitor** runs sets of checks periodically on all the repositories registered in the database. These checks are run *every hour*, provided the repository has changed since the last time it was checked. In the case of repositories that don't change often, we make sure that they are checked at least *once a day* anyway. This way we keep reports up to date with the latest checks additions and improvements.

Checks are organized in `check sets`. Each `check set` defines a number of checks that will be run on the repository and one or more `check sets` can be applied to a single repository. At the moment the following sets are supported: `code`, `code-lite`, `community` and `docs`. The set of checks run for each one are as follows:

- **code** (recommended for projects' primary code repository)

  - Documentation / Changelog
  - Documentation / Contributing
  - Documentation / Maintainers
  - Documentation / Readme
  - License
  - License / Approved
  - License / Scanning
  - Best practices / Artifact Hub badge
  - Best practices / CLA
  - Best practices / DCO
  - Best practices / OpenSSF best practices badge
  - Best practices / OpenSSF Scorecard badge
  - Best practices / Recent release
  - Security / Binary artifacts
  - Security / Code review
  - Security / Dangerous workflow
  - Security / Dependency update tool
  - Security / Insights
  - Security / Maintained
  - Security / SBOM
  - Security / Policy
  - Security / Signed releases
  - Security / Token permissions

- **code-lite** (subset of *code*, recommended for secondary code repositories)

  - Documentation / Contributing
  - Documentation / Maintainers
  - Documentation / Readme
  - License
  - License / Approved
  - Best practices / CLA
  - Best practices / DCO
  - Best practices / Recent release

- **community** (recommended for repositories with community content)

  - Documentation / Adopters
  - Documentation / Code of conduct
  - Documentation / Contributing
  - Documentation / Governance
  - Documentation / Readme
  - Documentation / Roadmap
  - Documentation / Summary table
  - Documentation / Website
  - Best practices / Community meeting
  - Best practices / GitHub discussions
  - Best practices / Slack presence
  - Security / Policy
  - Legal / Trademark disclaimer

- **docs** (recommended for other documentation repositories)

  - Documentation / Readme
  - License
  - License / Approved

Many checks rely on checking that certain files exists on a given path. Even though most of these checks support a number of variants, sometimes this won't work for some projects that may be using a different repository layout. In those cases, the recommended approach is to add a section to the `README` file of the repository pointing users to the document location. This will help users discovering this information and will make CLOMonitor happy :) At the moment we support detecting headers as well as links in `README` files that follow some patterns. Please see the reference below for more information on each case. Some projects have already proceeded this way successfully: [Kubernetes clomonitor PR](https://github.com/kubernetes/kubernetes/pull/108110), [KEDA clomonitor PR](https://github.com/kedacore/keda/pull/2704) and [Cilium clomonitor PR](https://github.com/cilium/cilium/pull/19037).

For more details about how each of the checks are performed, please see the reference below. Note that **CLOMonitor** does not follow symlinks when reading files content. If you find that any of the checks isn't working as expected or you have ideas about how to improve them please [file an issue](https://github.com/cncf/clomonitor/issues) or [open a discussion](https://github.com/cncf/clomonitor/discussions) in GitHub.

## Exemptions

Sometimes some of the checks may not be applicable to a repository (i.e. Artifact Hub badge in the Kubernetes project). In those cases, it's possible to declare an exemption in the [`.clomonitor.yml`](https://github.com/cncf/clomonitor/blob/main/docs/metadata/.clomonitor.yml) metadata file.

Each of the exemptions declared must include a reason that justifies it. Exempt checks will be specially marked in the UI, and the provided justification will be displayed to let users know why the check was not required in this case.

The checks identifiers (**ID**) required to declare an exemption can be found in the reference below.

## Documentation

### Adopters

**ID**: `adopters`

List of organizations using this project in production or at stages of testing.

This check passes if:

- An adopters *file* is found in the repository. Globs used:

```sh
"adopters*"
"users*"

CASE SENSITIVE: false
```

- An adopters *reference* is found in the repository's `README` file. This can be in the form of a **title header** or a link. Regexps used:

```sh
"(?im)^#+.*adopters.*$"
"(?im)^adopters$"
"(?i)\[.*adopters.*\]\(.*\)"
```

### Changelog

**ID**: `changelog`

A curated, chronologically ordered list of notable changes for each version.

This check passes if:

- A changelog *file* is found in the repository. Globs used:

```sh
"changelog*"

CASE SENSITIVE: false
```

- A changelog *reference* is found in the repository's `README` file. This can be in the form of a **title header** or a link. Regexps used:

```sh
"(?im)^#+.*changelog.*$"
"(?im)^changelog$"
"(?i)\[.*changelog.*\]\(.*\)"
```

- A changelog *reference* is found in the last Github release content body. Regexps used:

```sh
"(?i)changelog"
"(?i)changes"
```

### Code of conduct

**ID**: `code_of_conduct`

Adopt a code of conduct to define community standards, signal a welcoming and inclusive project, and outline procedures for handling abuse.

This check passes if:

- A code of conduct *file* is found in the repository. Globs used:

```sh
"code*of*conduct*"
".github/code*of*conduct*"
"docs/code*of*conduct*"

CASE SENSITIVE: false
```

- A code of conduct *reference* is found in the repository's `README` file. This can be in the form of a **title header** or a link. Regexps used:

```sh
"(?im)^#+.*code of conduct.*$"
"(?im)^code of conduct$"
"(?i)\[.*code of conduct.*\]\(.*\)"
```

- A code of conduct *file* is found in Github (in the [`.github` default community health files repository](https://docs.github.com/en/communities/setting-up-your-project-for-healthy-contributions/creating-a-default-community-health-file), for example).

### Contributing

**ID**: `contributing`

A contributing file in your repository provides potential project contributors with a short guide to how they can help with your project.

This check passes if:

- A contributing *file* is found in the repository. Globs used:

```sh
"contributing*"
".github/contributing*"
"docs/contributing*"

CASE SENSITIVE: false
```

- A contributing *reference* is found in the repository's `README` file. This can be in the form of a **title header** or a link. Regexps used:

```sh
"(?im)^#+.*contributing.*$"
"(?im)^contributing$"
"(?i)\[.*contributing.*\]\(.*\)"
```

- A contributing file is found in the [`.github` default community health files repository](https://docs.github.com/en/communities/setting-up-your-project-for-healthy-contributions/creating-a-default-community-health-file).

### Governance

**ID**: `governance`

Document that explains how the governance and committer process works in the repository.

This check passes if:

- A governance *file* is found in the repository. Globs used:

```sh
"governance*"
"docs/governance*"

CASE SENSITIVE: false
```

- A governance *reference* is found in the repository's `README` file. This can be in the form of a **title header** or a link. Regexps used:

```sh
"(?im)^#+.*governance.*$"
"(?im)^governance$"
"(?i)\[.*governance.*\]\(.*\)"
```

### Maintainers

**ID**: `maintainers`

The maintainers file contains a list of the current maintainers of the repository.

This check passes if:

- A maintainers *file* is found in the repository. Globs used:

```sh
"maintainers*"
"docs/maintainers*"
"owners*"
"docs/owners*"
"codeowners*"
".github/codeowners*"
"docs/codeowners*"

CASE SENSITIVE: false
```

- A maintainers *reference* is found in the repository's `README` file. This can be in the form of a **title header** or a link. Regexps used:

```sh
"(?im)^#+.*maintainers.*$"
"(?im)^maintainers$"
"(?i)\[.*maintainers.*\]\(.*\)"
```

### Readme

**ID**: `readme`

The readme file introduces and explains a project. It contains information that is commonly required to understand what the project is about.

This check passes if:

- A readme *file* is found in the repository. Globs used:

```sh
"README*"
".github/README*"
"docs/README*"

CASE SENSITIVE: true
```

### Roadmap

**ID**: `roadmap`

Defines a high-level overview of the project's goals and deliverables ideally presented on a timeline.

This check passes if:

- A roadmap *file* is found in the repository. Globs used:

```sh
"roadmap*"

CASE SENSITIVE: false
```

- A roadmap *reference* is found in the repository's `README` file. This can be in the form of a **title header** or a link. Regexps used:

```sh
"(?im)^#+.*roadmap.*$"
"(?im)^roadmap$"
"(?i)\[.*roadmap.*\]\(.*\)"
```

### Summary Table

**ID**: `summary_table`

The [Projects Summary Table](https://landscape.cncf.io/summary) is a CNCF Business Value Subcommittee initiative to supplement the CNCF Landscape and include further information about CNCF projects for the wider Cloud Native community.

This check passes if:

- At least *one* of the [summary_* fields](https://github.com/cncf/landscape/blob/master/readme_summary.md#using-your-own-tooling) has been set in the project's *extra* section in the [Landscape yaml file](https://github.com/cncf/landscape/blob/master/landscape.yml).

### Website

**ID**: `website`

A url that users can visit to learn more about your project.

This check passes if:

- A website *url* is configured in the Github repository.

## License

### SPDX id

**ID**: `license_spdx_id`

Identifier detected from the license file provided.

This check passes if:

- A license *file* is found in the repository and we can detect the license used. Globs used:

```sh
"LICENSE*"
"COPYING*"

CASE SENSITIVE: true
```

- A license SPDX id can be obtained from Github.

### Approved license

**ID**: `license_approved`

Whether the repository uses an approved license or not.

This check passes if:

- The license identified matches any of the following:

```sh
"Apache-2.0"
"BSD-2-Clause"
"BSD-2-Clause-FreeBSD"
"BSD-3-Clause"
"ISC"
"MIT"
"PostgreSQL"
"Python-2.0"
"X11"
"Zlib"
```

### License scanning

**ID**: `license_scanning`

License scanning software scans and automatically identifies, manages and addresses open source licensing issues.

This check passes if:

- A `FOSSA` or `Snyk` link is found in the repository's `README` file. Regexps used:

```sh
"(https://app.fossa.(?:io|com)/projects/[^"'\)]+)"
"(https://snyk.io/test/github/[^/]+/[^/"]+)"
```

- A *link* pointing to the license scanning results is provided in the [.clomonitor.yml](https://github.com/cncf/clomonitor/blob/main/docs/metadata/.clomonitor.yml) metadata file.

## Best practices

### Artifact Hub badge

**ID**: `artifacthub_badge`

Projects can list their content on Artifact Hub to improve their discoverability.

This check passes if:

- An `Artifact Hub` badge is found in the repository's `README` file. Regexps used:

```sh
"(https://artifacthub.io/packages/[^"'\)]+)"
```

### Contributor license agreement

**ID**: `cla`

Defines the terms under which intellectual property has been contributed to a company/project.

This check passes if:

- A CLA check is found in the latest merged PR on Github. Regexps used:

```sh
"(?i)cncf-cla"
"(?i)cla/linuxfoundation"
"(?i)easycla"
"(?i)license/cla"
"(?i)cla/google"
```

NOTE: *this check will be automatically marked as exempt if the DCO check passes and this one does not*.

### Community meeting

**ID**: `community_meeting`

Community meetings are often held to engage community members, hear more voices and get more viewpoints.

This check passes if:

- A *reference* to the community meeting is found in the repository's `README` file. Regexps used:

```sh
"(?im)^#+.*meeting.*$"
"(?i)(community|developer|development|working group) \[?(call|event|meeting|session)"
"(?i)(weekly|biweekly|monthly) \[?meeting"
"(?i)meeting minutes"
```

### Developer Certificate of Origin

**ID**: `dco`

Mechanism for contributors to certify that they wrote or have the right to submit the code they are contributing.

This check passes if:

- The last commits in the repository have the DCO signature (*Signed-off-by*). Merge pull request and merge branch commits are ignored for this check.

- A DCO check is found in the latest merged PR on Github. Regexps used:

```sh
"(?i)dco"
```

NOTE: *this check will be automatically marked as exempt if the CLA check passes and this one does not*.

### GitHub discussions

**ID**: `github_discussions`

Projects should enable GitHub discussions in their repositories.

This check passes if:

- A discussion that is less than one year old is found on Github.

### OpenSSF best practices badge

**ID**: `openssf_badge`

The Open Source Security Foundation (OpenSSF) Best Practices badge is a way for Free/Libre and Open Source Software (FLOSS) projects to show that they follow best practices.

This check passes if:

- An `OpenSSF` best practices badge is found in the repository's `README` file. Regexps used:

```sh
"(https://www.bestpractices.dev/projects/\d+)"
"(https://bestpractices.coreinfrastructure.org/projects/\d+)"
```

### OpenSSF Scorecard badge

**ID**: `openssf_scorecard_badge`

Scorecard assesses open source projects for security risks through a series of automated checks. For more information about the Scorecard badge please see <https://github.com/marketplace/actions/ossf-scorecard-action#scorecard-badge>.

This check passes if:

- An `OpenSSF` Scorecard badge is found in the repository's `README` file. Regexps used:

```sh
"(https://api.securityscorecards.dev/projects/github.com/[^/]+/[^/]+)/badge"
```

### Recent release

**ID**: `recent_release`

The project should have released at least one version in the last year.

This check passes if:

- A release that is less than one year old is found on Github.

### Slack presence

**ID**: `slack_presence`

Projects should have presence in the CNCF Slack or Kubernetes Slack.

This check passes if:

- A *reference* to the CNCF Slack or Kubernetes Slack is found in the repository's `README` file. Regexps used:

```sh
"(?i)https?://cloud-native.slack.com"
"(?i)https?://slack.cncf.io"
"(?i)https?://kubernetes.slack.com"
"(?i)https?://slack.k8s.io"
```

## Security

### Binary artifacts (from OpenSSF Scorecard)

**ID**: `binary_artifacts`

This check determines whether the project has generated executable (binary) artifacts in the source repository.

*This is an OpenSSF Scorecard check. For more details please see the [check documentation](https://github.com/ossf/scorecard/blob/main/docs/checks.md#binary-artifacts) in the ossf/scorecard repository.*

### Code review (from OpenSSF Scorecard)

**ID**: `code_review`

This check determines whether the project requires code review before pull requests (merge requests) are merged.

*This is an OpenSSF Scorecard check. For more details please see the [check documentation](https://github.com/ossf/scorecard/blob/main/docs/checks.md#code-review) in the ossf/scorecard repository.*

### Dangerous workflow (from OpenSSF Scorecard)

**ID**: `dangerous_workflow`

This check determines whether the project's GitHub Action workflows has dangerous code patterns.

*This is an OpenSSF Scorecard check. For more details please see the [check documentation](https://github.com/ossf/scorecard/blob/main/docs/checks.md#dangerous-workflow) in the ossf/scorecard repository.*

### Dependencies policy

**ID**: `dependencies_policy`

Project should provide a dependencies policy that describes how dependencies are consumed and updated.

This check passes if:

- The url of the dependencies policy is available in the `dependencies > env-dependencies-policy` section of the [OpenSSF Security Insights](https://github.com/ossf/security-insights-spec/blob/v1.0.0/specification.md) *manifest file* (`SECURITY-INSIGHTS.yml`) that should be located at the root of the repository.

### Dependency update tool (from OpenSSF Scorecard)

**ID**: `dependency_update_tool`

This check tries to determine if the project uses a dependency update tool, specifically [dependabot](https://docs.github.com/en/code-security/supply-chain-security/keeping-your-dependencies-updated-automatically/configuration-options-for-dependency-updates) or [renovatebot](https://docs.renovatebot.com/configuration-options/).

*This is an OpenSSF Scorecard check. For more details please see the [check documentation](https://github.com/ossf/scorecard/blob/main/docs/checks.md#dependency-update-tool) in the ossf/scorecard repository.*

### Maintained (from OpenSSF Scorecard)

**ID**: `maintained`

This check determines whether the project is actively maintained.

*This is an OpenSSF Scorecard check. For more details please see the [check documentation](https://github.com/ossf/scorecard/blob/main/docs/checks.md#maintained) in the ossf/scorecard repository.*

### Security insights

**ID**: `security_insights`

Projects should provide an [OpenSSF Security Insights](https://github.com/ossf/security-insights-spec/blob/v1.0.0/specification.md) manifest file.

This check passes if:

- A valid OpenSSF Security Insights *manifest file* (`SECURITY-INSIGHTS.yml`) is found at the root of the repository.

### Security policy

**ID**: `security_policy`

Clearly documented security processes explaining how to report security issues to the project.

This check passes if:

- A security policy *file* is found in the repository. Globs used:

```sh
"security*"
".github/security*"
"docs/security*"

CASE SENSITIVE: false
```

- A security policy *reference* is found in the repository's `README` file. This can be in the form of a **title header** or a link. Regexps used:

```sh
"(?im)^#+.*security.*$"
"(?im)^security$"
"(?i)\[.*security.*\]\(.*\)"
```

- A security policy *file* is found in Github (in the [`.github` default community health files repository](https://docs.github.com/en/communities/setting-up-your-project-for-healthy-contributions/creating-a-default-community-health-file), for example).

### Signed releases (from OpenSSF Scorecard)

**ID**: `signed_releases`

This check tries to determine if the project cryptographically signs release artifacts.

*This is an OpenSSF Scorecard check. For more details please see the [check documentation](https://github.com/ossf/scorecard/blob/main/docs/checks.md#signed-releases) in the ossf/scorecard repository.*

### Software bill of materials (SBOM)

**ID**: `sbom`

List of components in a piece of software, including licenses, versions, etc.

This check passes if:

- The latest release on Github includes an asset which name contains *sbom*. Regexps used:

```sh
"(?i)sbom"
```

- The repository's `README` file contains a *SBOM* section that explains where they are published to, format used, etc. Regexps used to locate the *title header*:

```sh
"(?im)^#+.*sbom.*$"
"(?im)^#+.*software bill of materials.*$"
"(?im)^sbom$"
"(?im)^software bill of materials$"
```

### Token permissions (from OpenSSF Scorecard)

**ID**: `token_permissions`

This check determines whether the project's automated workflows tokens are set to read-only by default.

*This is an OpenSSF Scorecard check. For more details please see the [check documentation](https://github.com/ossf/scorecard/blob/main/docs/checks.md#token-permissions) in the ossf/scorecard repository.*

## Legal

### Trademark disclaimer

**ID**: `trademark_disclaimer`

Projects sites should have the Linux Foundation trademark disclaimer.

This check passes if:

- The Linux Foundation trademark disclaimer is found in the content of the website configured in Github. Regexps used:

```sh
"https://(?:w{3}\.)?linuxfoundation.org/(?:legal/)?trademark-usage"
"The Linux Foundation.* has registered trademarks and uses trademarks"
```

Note: This check currently only supports static web sites where the content is delivered in an HTML page to the browser. If you use a dynamic site (e.g., React, Angular), your repo may want to set an [exemption](#exemptions) for this check ID.
