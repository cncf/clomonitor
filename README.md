# CLOMonitor

[![CI](https://github.com/cncf/clomonitor/workflows/CI/badge.svg)](https://github.com/cncf/clomonitor/actions?query=workflow%3ACI)
[![Gitpod Ready-to-Code](https://img.shields.io/badge/Gitpod-ready--to--code-blue?logo=gitpod)](https://gitpod.io/#https://github.com/cncf/clomonitor)

[**CLOMonitor**](https://clomonitor.io) is a tool that periodically checks open source projects repositories to verify they meet a certain project health best practices.

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

## Checks

**CLOMonitor** runs sets of checks periodically on all the repositories registered in the database. These checks are run *every hour*, provided the repository has changed since the last time it was checked. In the case of repositories that don't change often, we make sure that they are checked at least *once a day* anyway. This way we keep reports up to date with the latest checks additions and improvements.

Checks are organized in `check sets`. Each `check set` defines a number of checks that will be run on the repository and one or more `check sets` can be applied to a single repository. At the moment the following sets are supported: `code`, `code-lite`, `community` and `docs`. Please see the [checks documentation](./docs/checks.md) for more details.

## Linter CLI

The CLOMonitor's linter can also be run locally or from CI workflows. At the moment we are publishing a [Docker image](https://gallery.ecr.aws/clomonitor/linter) with the tool, but we'll be publishing binaries for different platforms soon. You can find an example of how to integrate it with Github Actions [in the Artifact Hub repository](https://github.com/artifacthub/hub/blob/a25d69235ef9a196aa905e160c99977b692d5e34/.github/workflows/ci.yml#L40-L49).

```sh
$ clomonitor-linter --help
clomonitor-linter 0.5.0
A linter for open source projects repositories

USAGE:
    clomonitor-linter [OPTIONS] --url <URL>

OPTIONS:
        --check-set <CHECK_SET>      Sets of checks to run [default: code community] [possible
                                     values: code, code-lite, community, docs]
    -h, --help                       Print help information
        --pass-score <PASS_SCORE>    Linter pass score [default: 80]
        --path <PATH>                Repository root path [default: .]
        --url <URL>                  Repository url [https://github.com/org/repo] (required for some
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
