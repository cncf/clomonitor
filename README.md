# CLOMonitor

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

[clomonitor.io](https://clomonitor.io) lists most of the projects in the [CNCF](https://www.cncf.io/projects/), [LF AI & DATA](https://lfaidata.foundation/projects/) and [CDF](https://cd.foundation/projects/) foundations. If you notice that a project that belongs to any of those foundations is missing or has some incorrect or missing information, please feel free to submit a pull request with your suggested changes. The YAML data files for the registered foundations can be found in this repository, at the [/data](https://github.com/cncf/clomonitor/tree/main/data) path. **CLOMonitor** checks periodically those data files and applies the corresponding changes as needed.

Every project featured on [clomonitor.io](https://clomonitor.io) will be provided with a badge and report summary that is ready for use in your project repos. Simply click the menu dropdown on your project page and copy+paste the code snippet into your markdown as desired. An example can be seen in the [image shown above](docs/screenshots/embed-report-light.png).

## Checks

**CLOMonitor** runs sets of checks periodically on all the repositories registered in the database. These checks are run *every hour*, provided the repository has changed since the last time it was checked. In the case of repositories that don't change often, we make sure that they are checked at least *once a day* anyway. This way we keep reports up to date with the latest checks additions and improvements.

Checks are organized in `check sets`. Each `check set` defines a number of checks that will be run on the repository and one or more `check sets` can be applied to a single repository. At the moment the following sets are supported: `code`, `code-lite`, `community` and `docs`. Please see the [checks documentation](./docs/checks.md) for more details.

## Linter CLI

The CLOMonitor's linter can also be run locally or from CI workflows. This can be done by using the [container image](https://gallery.ecr.aws/clomonitor/linter) provided or by building the CLI tool from the source.

CLOMonitor delegates some of the security checks to [OpenSSF Scorecard](https://github.com/ossf/scorecard). When building from the source, you'll need to [install it](https://github.com/ossf/scorecard#installation) before running `clomonitor-linter` locally. The container image already includes the `scorecard` binary, so if you opt for using it you are ready to go.

Both CLOMonitor and Scorecard use the GitHub GraphQL API for some checks, which requires authentication. A GitHub token (with `public_repo` scope) **must** be provided via the `GITHUB_TOKEN` environment variable to authenticate those requests.

### Using Docker

You can run the linter CLI tool from Docker by running the following command:

```text
$ export GITHUB_TOKEN=<your token>

$ docker run -it \
  --volume $PWD:/repo \
  --env GITHUB_TOKEN=$GITHUB_TOKEN \
  ghcr.io/cncf/clomonitor/linter clomonitor-linter \
    --path /repo \
    --url https://github.com/<org>/<repo>
```

Note: *the command assumes the current working directory is the repo you would like to lint. Please adjust the repo url as needed.*

#### CI workflow integration

An example of how to integrate CLOMonitor's linter with GitHub Actions can be found [in the Artifact Hub repository](https://github.com/artifacthub/hub/blob/c73dafa519020415927665e14fb6eac1066120eb/.github/workflows/ci.yml#L46-L57).

### Building from source

You can also build the CLOMonitor's linter CLI tool from the source by using [Cargo](https://rustup.rs), the Rust package manager:

```text
$ cargo install --git https://github.com/cncf/clomonitor clomonitor-linter

$ clomonitor-linter --help
```

## Contributing

Please see [CONTRIBUTING.md](./CONTRIBUTING.md) for more details.

You can find some general information about how to work on this repo in the [architecture](./docs/architecture.md) and the [development environment setup](./docs/dev.md) documents.

## Code of Conduct

This project follows the [CNCF Code of Conduct](https://github.com/cncf/foundation/blob/master/code-of-conduct.md).

## License

CLOMonitor is an Open Source project licensed under the [Apache License 2.0](https://www.apache.org/licenses/LICENSE-2.0).
