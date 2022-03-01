# Architecture

This document describes the architecture of **CLOMonitor**, detailing each of the components, what they do and where they are located in the source repository.

## Repository layout

The following directories present at the top level of the repository represent some of the key locations in the codebase:

```sh
clomonitor
├── .github
├── chart
├── clomonitor-apiserver
├── clomonitor-core
├── clomonitor-linter
├── clomonitor-tracker
├── database
├── docs
└── web
```

- **.github:** contains the Github Actions workflows.

- **chart:** contains the CLOMonitor Helm chart, which is the recommended installation method.

- **clomonitor-apiserver** contains the source code of the `apiserver` backend component.

- **clomonitor-core:** contains the source code of the `core backend modules`, like linter and score.

- **clomonitor-linter:** contains the source code of `linter CLI` tool.

- **clomonitor-tracker:** contains the source code of the `tracker` backend component.

- **database:** contains all code related to the database layer, such as the schema migrations, functions and tests.

- **docs:** contains the project documentation.

- **web:** contains the source code of the `web application`.

## Layers

CLOMonitor is structured in multiple layers, each of them providing a set of services to adjacent layers.

- **Database:** this layer provides data services, controlling the database [schema and its migrations](https://github.com/cncf/clomonitor/tree/main/database/migrations/schema) and providing a set of [functions](https://github.com/cncf/clomonitor/tree/main/database/migrations/functions) that will act as an API for outer layers, abstracting them in many cases from the internal database structure. CLOMonitor uses PostgreSQL as datastore. Please see the [database](#database) section for more details.

- **Core library:** this layer represents a set of Rust APIs that allow performing core operations supported by CLOMonitor, such as linting a repository or calculating scores. Please see the [core library](#core-library) section for more details.

- **Backend applications:** this layer represents the applications that form the backend: `apiserver` and `tracker`. These applications rely on the `database` and `core library` layers to perform their tasks. Please see the [backend applications](#backend-applications) section for more details.

- **Linter CLI:** this layer represents a CLI tool that allows projects to lint their repositories locally or from their CI workflows. Please see the [linter CLI](#linter-cli) section for more details.

- **Web application:** this layer represents the CLOMonitor's web user interface. It uses the HTTP API exposed from the `apiserver` to interact with the backend. Please see the [web application](#web-application) section for more details.

## Database

The `database` layer is defined by the database [schema](https://github.com/cncf/clomonitor/tree/main/database/migrations/schema) and a set of [functions](https://github.com/cncf/clomonitor/tree/main/database/migrations/functions), which are handled using migrations. Migrations use [Tern](https://github.com/jackc/tern), and are automatically applied during the installation and upgrades by a Kubernetes [job](https://github.com/cncf/clomonitor/blob/main/chart/templates/dbmigrator_install_job.yaml) named `dbmigrator`. There are [unit tests](https://github.com/cncf/clomonitor/tree/main/database/tests) available for both the schema and the functions.

```sh
database
├── migrations
│   ├── functions
│   │   └── ...
│   └── schema
│       └── ...
└── tests
    ├── functions
    │   └── ...
    └── schema
        └── ...
```

## Core library

This layer represents a set of **Rust APIs** that provide some core functionality to other layers, like the `backend applications` or the `CLI tool`.

It's composed of two modules:

- **linter:** this module implements the core linting functionality of CLOMonitor. All checks currently done by CLOMonitor are handled by this module, and both the `CLI tool` and the `tracker` rely on it. The linter is able to handle multiple kinds of repositories, as each may require a different set of checks. At the moment two are supported: `primary` and `secondary`. The primary kind is used for a project's main repository, and it performs a more in depth checking. The secondary kind is used for other repositories in the project and only performs a small subset of the checks.

- **score:** this module is in charge of scoring reports produced by the linter. The linter will produce different reports for each of the kinds supported, and each of the reports will be scored differently as well. In addition to the reports' scoring functionality, this module provides some score related features as well, like rating a given score or merging multiple scores.

## Backend applications

The backend applications are `apiserver` and `tracker`. They are located in the `clomonitor-apiserver` and `clomonitor-tracker` directories respectively. Each of the applications' directory contains a `Dockerfile` that will be used to build the corresponding Docker image.

```sh
.
├── clomonitor-apiserver
│   ├── Cargo.toml
│   ├── Dockerfile
│   ├── src
│   └── templates
└── clomonitor-tracker
    ├── Cargo.toml
    ├── Dockerfile
    └── src
```

- **apiserver:** this component provides an HTTP API that exposes some endpoints used by the web application layer, plus some extra functionality like badges configuration, reports summary, etc. It is also in charge of serving the web application static assets.

- **tracker:** this component is in charge of linting and scoring all projects and repositories registered in the database. It's launched periodically from a Kubernetes [cronjob](https://github.com/cncf/clomonitor/blob/main/chart/templates/tracker_cronjob.yaml).

## Linter CLI

The linter CLI tool allows projects to lint their repositories locally or from their CI workflows. It generates a report by using the linter module available in the core library, it scores it and prints the results nicely.

```sh
clomonitor-linter 0.1.0
A linter for CNCF projects repositories

USAGE:
    clomonitor-linter [OPTIONS] --url <URL>

OPTIONS:
    -h, --help                       Print help information
        --kind <KIND>                Repository kind [default: primary] [possible values: primary,
                                     secondary]
        --pass-score <PASS_SCORE>    Linter pass score [default: 80]
        --path <PATH>                Repository root path [default: .]
        --url <URL>                  Repository url [https://github.com/org/repo] (required for some
                                     remote checks in Github)
    -V, --version                    Print version information
```

Please see this [discussion](https://github.com/cncf/clomonitor/discussions/20) for more information and some screenshots.

## Web application

The CLOMonitor's user interface is a single page application written in TypeScript using React. Its source code can be found in the `web` directory.

```sh
web
├── public
└── src
    ├── api
    ├── context
    ├── hooks
    ├── layout
    ├── styles
    └── utils
```

- **public:** contains the base `index.html` file as well as some static assets, like images.

- **src/api:** contains a wrapper to interact with the HTTP API exposed by the `apiserver`.

- **src/context:** context used for the preferences across the entire app.

- **src/hooks:** contains some custom React hooks.

- **src/layout:** contains all React components. They are organized in different folders corresponding to the section of the UI they belong to.

- **src/styles:** contains the stylesheets for the light and dark themes.

- **src/utils:** contains some utilities used by the components.

## Development environment setup

For more information about how to setup your development environment, please see [this document](https://github.com/cncf/clomonitor/blob/main/docs/dev.md).
