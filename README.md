# CLOMonitor

[![CI](https://github.com/cncf/clomonitor/workflows/CI/badge.svg)](https://github.com/cncf/clomonitor/actions?query=workflow%3ACI)
[![Gitpod Ready-to-Code](https://img.shields.io/badge/Gitpod-ready--to--code-blue?logo=gitpod)](https://gitpod.io/#https://github.com/cncf/clomonitor)

[**CLOMonitor**](https://clomonitor.io) is a tool that periodically checks open source projects repositories to verify they meet certain project health best practices.

<br/>
<table>
    <tr>
        <td width="50%"><img src="docs/screenshots/search-light.png?raw=true"></td>
        <td width="50%"><img src="docs/screenshots/search-dark.png?raw=true"></td>
    </tr>
    <tr>
        <td width="50%"><img src="docs/screenshots/project-light.png?raw=true"></td>
        <td width="50%"><img src="docs/screenshots/project-dark.png?raw=true"></td>
    </tr>
    <tr>
        <td width="50%"><img src="docs/screenshots/stats-light.png?raw=true"></td>
        <td width="50%"><img src="docs/screenshots/embed-report-light.png?raw=true"></td>
    </tr>
</table>

## Projects

[clomonitor.io](https://clomonitor.io) lists most of the projects in the [CNCF](https://www.cncf.io/projects/) and [LF AI & DATA](https://lfaidata.foundation/projects/) foundations. If you notice that a project that belongs to any of those foundations is missing or has some incorrect or missing information, please feel free to submit a pull request with your suggested changes. The YAML data files for the registered foundations can be found in this repository, at the [/data](https://github.com/cncf/clomonitor/tree/main/data) path. **CLOMonitor** checks periodically those data files and applies the corresponding changes as needed.

## Checks

**CLOMonitor** runs sets of checks periodically on all the repositories registered in the database. These checks are run *every hour*, provided the repository has changed since the last time it was checked. In the case of repositories that don't change often, we make sure that they are checked at least *once a day* anyway. This way we keep reports up to date with the latest checks additions and improvements.

Checks are organized in `check sets`. Each `check set` defines a number of checks that will be run on the repository and one or more `check sets` can be applied to a single repository. At the moment the following sets are supported: `code`, `code-lite`, `community` and `docs`. Please see the [checks documentation](./docs/checks.md) for more details.

## Linter CLI

The CLOMonitor's linter can also be run locally or from CI workflows. You can build it from source using [Cargo](https://rustup.rs), the Rust package manager:

```sh
cargo install --git https://github.com/cncf/clomonitor clomonitor-linter
```

Alternatively, you can use the published [Docker image](https://gallery.ecr.aws/clomonitor/linter). An example of how to integrate CLOMonitor's linter with Github Actions can be found [in the Artifact Hub repository](https://github.com/artifacthub/hub/blob/c73dafa519020415927665e14fb6eac1066120eb/.github/workflows/ci.yml#L46-L57).

CLOMonitor delegates some of the security checks to [OpenSSF Scorecard](https://github.com/ossf/scorecard), so you'll need to [install it](https://github.com/ossf/scorecard#installation) before running `clomonitor-linter` locally. Both CLOMonitor and Scorecard use the Github GraphQL API for some checks, which requires authentication. A Github token (with `public_repo` scope) **must** be provided via the `GITHUB_TOKEN` environment variable to authenticate those requests.

```text
$ export GITHUB_TOKEN=<your token>

$ clomonitor-linter --help
clomonitor-linter
Checks repository to verify it meets certain project health best practices

USAGE:
    clomonitor-linter [OPTIONS] --path <PATH> --url <URL>

OPTIONS:
        --check-set <CHECK_SET>      Sets of checks to run [default: code community] [possible
                                     values: code, code-lite, community, docs]
        --format <FORMAT>            Output format [default: table] [possible values: json, table]
    -h, --help                       Print help information
        --pass-score <PASS_SCORE>    Linter pass score [default: 75]
        --path <PATH>                Repository local path (used for checks that can be done
                                     locally)
        --url <URL>                  Repository url [https://github.com/org/repo] (used for some
                                     GitHub remote checks)
    -V, --version                    Print version information
```

## Contributing

Please see [CONTRIBUTING.md](./CONTRIBUTING.md) for more details.

You can find some general information about how to work on this repo in the [architecture](./docs/architecture.md) and the [development environment setup](./docs/dev.md) documents.

## Code of Conduct

This project follows the [CNCF Code of Conduct](https://github.com/cncf/foundation/blob/master/code-of-conduct.md).

## License

CLOMonitor is an Open Source project licensed under the [Apache License 2.0](https://www.apache.org/licenses/LICENSE-2.0).
