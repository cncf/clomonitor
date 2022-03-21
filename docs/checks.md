# Checks

**CLOMonitor** runs a set of checks periodically on all the repositories registered in the database. These checks are run *every hour*, provided the repository has changed since the last time it was checked. In the case of repositories that don't change often, we make sure that they are checked at least *once a day* anyway. This way we keep reports up to date with the latest checks additions and improvements.

The set of checks run depends on the repository kind. At the moment two kinds are supported: `primary` and `secondary`. Repositories of kind `primary` are checked more thoroughly, so it's important to make sure the right one has been marked as primary. If you find that the repository used as primary for your project is not correct, please [file an issue](https://github.com/cncf/clomonitor/issues) and we'll gladly update it. The `secondary` kind can be used for other project's repositories that'd be interesting to check as well.

Many checks rely on checking that certain files exists on a given path. Even though most of these checks support a number of variants, sometimes this won't work for some projects that may be using a different repository layout or organizing content in multiple repositories. In those cases, the recommended approach is to add a section in the `README` file of the primary repository pointing users to the document location. This will help users discovering this information and will make CLOMonitor happy :) At the moment we support detecting headers as well as links in `README` files that follow some patterns. Please see the reference below for more information on each case. Some projects have already proceeded this way successfully: [Kubernetes clomonitor PR](https://github.com/kubernetes/kubernetes/pull/108110), [KEDA clomonitor PR](https://github.com/kedacore/keda/pull/2704) and [Cilium clomonitor PR](https://github.com/cilium/cilium/pull/19037).

For more details about how each of the checks are performed, please see the reference below. If you find that any of the checks isn't working as expected or you have ideas about how to improve them please [file an issue](https://github.com/cncf/clomonitor/issues) or [open a discussion](https://github.com/cncf/clomonitor/discussions) in Github.

## Exemptions

Sometimes some of the checks may not be applicable to a repository (i.e. Artifact Hub badge in the Kubernetes project). In those cases, it's possible to declare an exemption in the [.clomonitor.yml](https://github.com/cncf/clomonitor/blob/main/docs/metadata/.clomonitor.yml) metadata file.

Each of the exemptions declared must include a reason that justifies it. Exempt checks will be specially marked in the UI, and the provided justification will be displayed to let users know why the check was not required in this case.

The checks identifiers (**ID**) required to declare an exemption can be found in the reference below.

## Documentation

### Adopters

`**ID**: *adopters*`

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

`**ID**: *changelog*`

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

`**ID**: *code_of_conduct*`

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

- A code of conduct file is found in the [`.github` default community health files repository](https://docs.github.com/en/communities/setting-up-your-project-for-healthy-contributions/creating-a-default-community-health-file).

### Contributing

`**ID**: *contributing*`

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

`**ID**: *governance*`

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

`**ID**: *maintainers*`

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

`**ID**: *readme*`

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

`**ID**: *roadmap*`

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

### Website

`**ID**: *website*`

A url that users can visit to learn more about your project.

This check passes if:

- A website *url* is configured in the Github repository.

## License

### SPDX id

`**ID**: *license_spdx_id*`

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

`**ID**: *license_approved*`

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

`**ID**: *license_scanning*`

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

`**ID**: *artifacthub_badge*`

Projects can list their content on Artifact Hub to improve their discoverability.

This check passes if:

- An `Artifact Hub` badge is found in the repository's `README` file. Regexps used:

```sh
"https://artifacthub.io/badge/repository/.*
```

### Community meeting

`**ID**: *community_meeting*`

Community meetings are often held to engage community members, hear more voices and get more viewpoints.

This check passes if:

- A *reference* to the community meeting is found in the repository's `README` file. Regexps used:

```sh
"(?i)(community|developer|development) \[?(call|event|meeting|session)"
"(?i)(weekly|biweekly|monthly) \[?meeting"
"(?i)meeting minutes"
```

### Developer Certificate of Origin

`**ID**: *dco*`

Mechanism for contributors to certify that they wrote or have the right to submit the code they are contributing.

This check passes if:

- The last commits in the repository have the DCO signature (*Signed-off-by*). Merge pull request and merge branch commits are ignored for this check.

- A *reference* to the DCO app is found in the last PR checks page on Github. Regexps used:

```sh
"[">]DCO[<"]"
```

### OpenSSF badge

`**ID**: *openssf_badge*`

The Open Source Security Foundation (OpenSSF) Best Practices badge is a way for Free/Libre and Open Source Software (FLOSS) projects to show that they follow best practices.

This check passes if:

- An `OpenSSF` (CII) badge is found in the repository's `README` file. Regexps used:

```sh
"https://bestpractices.coreinfrastructure.org/projects/\d+"
```

### Recent release

`**ID**: *recent_release*`

The project should have released at least one version in the last year.

This check passes if:

- A release that is less than one year old is found on Github.

### Slack presence

`**ID**: *slack_presence*`

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

### Security policy

`**ID**: *security_policy*`

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

- A security policy file is found in the [`.github` default community health files repository](https://docs.github.com/en/communities/setting-up-your-project-for-healthy-contributions/creating-a-default-community-health-file).

## Legal

### Trademark disclaimer

`**ID**: *trademark_disclaimer*`

Projects sites should have the Linux Foundation trademark disclaimer.

This check passes if:

- The Linux Foundation trademark disclaimer is found in the content of the website configured in Github. Regexps used:

```sh
"https://(?:w{3}\.)?linuxfoundation.org/trademark-usage"
"The Linux Foundation.* has registered trademarks and uses trademarks"
```
