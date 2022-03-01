# Development environment setup

This document will help you setup your development environment so that you can build, test and run CLOMonitor locally from source.

For more information about the CLOMonitor's architecture, please see [this document](https://github.com/cncf/clomonitor/blob/main/docs/architecture.md).

The instructions provided in this document rely on a set of [aliases](#aliases) available at the end. These aliases are used by some of the maintainers and are provided only as examples. Please feel free to adapt them to suit your needs. You may want to add them to your shell's configuration file so that they are loaded automatically.

To start, please clone the [CLOMonitor repository](https://github.com/cncf/clomonitor). If you plan to use the aliases mentioned above, you should set the `CLOMONITOR_SOURCE` variable to the path where you cloned the repository.

## Database

The datastore used by CLOMonitor is PostgreSQL. You can install it locally using your favorite OS package manager.

Once PostgreSQL is installed and its binaries are available in your `PATH`, we can initialize the database cluster and launch the database server:

```sh
clomonitor_db_init
clomonitor_db_server
```

Once the database server is up an running, we can create the `clomonitor` database and we'll be ready to go:

```sh
clomonitor_db_create
```

### Migrations

[Database migrations](https://github.com/cncf/clomonitor/tree/main/database/migrations) are managed using [Tern](https://github.com/jackc/tern). Please [install it](https://github.com/jackc/tern#installation) before proceeding. The database schema and functions are managed with migrations.

We need to create a configuration file so that Tern knows how to connect to our database. We'll create a file called `tern.conf` inside `~/.config/clomonitor` with the following content (please adjust if needed):

```ini
[database]
host = localhost
port = 5432
database = clomonitor
user = postgres
```

Now that the `clomonitor` database server is up and ready, we just need to apply all available migrations using the following command:

```sh
clomonitor_db_migrate
```

### Database tests

If you plan to do some work on the database layer, some extra setup is needed to be able to run the database tests. [Schema and database functions are tested](https://github.com/cncf/clomonitor/tree/main/database/tests) using the unit testing framework [pgTap](https://pgtap.org), so you need to [install](https://pgtap.org/documentation.html#installation) the pgTap PostgreSQL extension on your machine. To run the tests you will also need to install a perl tool called [pg_prove](https://pgtap.org/pg_prove.html) from CPAN (`cpan TAP::Parser::SourceHandler::pgTAP`).

Similarly to what we did during our initial database setup, we'll create a configuration file for Tern for the tests database in the same folder (`~/.config/clomonitor`), called `tern-tests.conf` with the following content (please adjust if needed):

```ini
[database]
host = localhost
port = 5432
database = clomonitor_tests
user = postgres
```

Once you have all the tooling required installed and the tests database set up, you can run all database tests as often as you need this way:

```sh
clomonitor_db_recreate_tests && clomonitor_db_tests
```

### Loading sample data

You can load some sample data by using the `psql` PostgreSQL client this way:

```sh
clomonitor_db_client
```

Once you've connected to the database, you can run the following commands -one by one- to load some organizations, projects and repositories (please make sure to adjust the path to the data files as needed):

```sql
\copy organization (organization_id, name, home_url, logo_url)
from '~/projects/clomonitor/database/data/organizations.csv'
with (format csv, header true, delimiter ';');

\copy project (project_id, maturity_id, category_id, name, display_name, description, logo_url, home_url, devstats_url, organization_id)
from '~/projects/clomonitor/database/data/projects.csv'
with (format csv, header true, delimiter ';');

\copy repository (repository_id, name, url, kind, project_id)
from '~/projects/clomonitor/database/data/repositories.csv'
with (format csv, header true, delimiter ';');
```

At this point our database is ready to launch our local instance of CLOMonitor and start doing some work on it.

## Backend

The backend is written in [Rust](https://www.rust-lang.org). Rust installation instructions can be found [here](https://www.rust-lang.org/tools/install).

To build the backend components, please run the command below:

```sh
cargo build
```

Even if you don't plan to do any work on the frontend, you will need to build it once if you want to interact with the CLOMonitor backend from the browser. To do this, you will have to install [yarn](https://yarnpkg.com/getting-started/install). Once you have it installed, you can build the frontend application this way:

```sh
cd web && yarn install
clomonitor_frontend_build
```

### API server

Once you have a working Rust development environment set up and the web application built, it's time to launch the `apiserver`. Before running it, we'll need to create a configuration file in `~/.config/clomonitor` named `apiserver.yaml` with the following content (please adjust `staticPath` as needed):

```yaml
db:
  host: localhost
  port: "5432"
  dbname: clomonitor
  user: postgres
  password: ""
apiserver:
  addr: 127.0.0.1:8000
  staticPath: /<YOUR_CLOMONITOR_LOCAL_PATH>/web/build
```

Now you can run the `apiserver`:

```sh
clomonitor_apiserver
```

The `apiserver` process launches an http server that serves the web application and the API that powers it. Once it is up and running, you can point your browser to [http://localhost:8000](http://localhost:8000) and you should see the CLOMonitor web application. Initially there won't be any projects listed on it, but we'll take care of that in the next section.

### Tracker

The `tracker` is a backend component in charge of linting the repositories registered in the database and updating the scores and ratings as needed. On production deployments, it is usually run periodically using a `cronjob` on Kubernetes. Locally, while developing, you can just run it as often as you need as any other CLI tool. The tracker requires the `git` command to be installed and available in your PATH.

If you opened the url suggested before, you probably noticed there were no projects listed yet. This happened because no repositories had been processed yet. When we set up the database, we loaded some sample repositories. To process them, we need to run the `tracker`.

Similarly to the `apiserver` server, the `tracker` can be configured using a `yaml` file. We'll create one in `~/.config/clomonitor` named `tracker.yaml` with the following content (please adjust as needed):

```yaml
db:
  host: localhost
  port: "5432"
  dbname: clomonitor
  user: postgres
  password: ""
creds:
  githubToken: <YOUR_GITHUB_TOKEN>
tracker:
  concurrency: 10
```

Some checks like *recent release* or *website* make some calls to the Github API. [Unauthenticated requests to the Github API are rate limited to 60 requests per hour](https://docs.github.com/en/rest/overview/resources-in-the-rest-api#rate-limiting), so you'll probably need to add your own Github token to the `tracker` configuration file to get up to 5000 (or 15000) requests per hour (no special permissions needed for it).

Once the configuration file is ready, it's time to launch the `tracker` for the first time:

```sh
clomonitor_tracker
```

Depending on the speed of your Internet connection and machine, this may take one or two minutes. The first time it runs all repositories will be linted. Subsequent runs will only lint repositories that have changed, so it'll be much faster. Once the tracker has completed, you should see projects in the web application.

### Linter CLI

In the section above we saw how the `tracker` is able to lint all repositories registered in the database. But sometimes it may be desirable to lint a single repository manually in an isolated way, maybe to quickly test some checks or to integrate with some other processes, like continuous integration or deployment tools. The `linter CLI` tool is designed to help in those scenarios.

If you are using the aliases provided below, you can run it this way:

```sh
clomonitor_linter --help
```

### Backend tests

You can run the backend tests by using `cargo`:

```sh
cargo test
```

## Frontend

The CLOMonitor frontend is a single page application written in [TypeScript](https://www.typescriptlang.org) using [React](https://reactjs.org).

In the backend section we mentioned how to install the frontend dependencies and build it. That should be enough if you are only going to work on the backend. However, if you are planning to do some work on the frontend, it's better to launch an additional server which will rebuild the web application as needed whenever a file is modified.

The frontend development server can be launched using the following command:

```sh
clomonitor_frontend_dev
```

That alias will launch an http server that will listen on the port `3000`. Once it's running, you can point your browser to [http://localhost:3000](http://localhost:3000) and you should see the CLOMonitor web application. The page will be automatically reloaded everytime you make a change in the frontend code. Build errors and build warnings will be visible in the console.

API calls will go to [http://localhost:8000](http://localhost:8000), so the [apiserver](#api-server) is expected to be up and running.

### Frontend tests and linter

You can use the command below to run all frontend tests:

```sh
clomonitor_frontend_tests
```

In addition to running the tests, you may also be interested in running the linter. To do that, you can run:

```sh
clomonitor_frontend_lint_fix
```

## Aliases

The following aliases are used by some of the maintainers and are provided only as examples. Please feel free to adapt them to suit your needs.

```sh
export CLOMONITOR_SOURCE=~/projects/clomonitor
export CLOMONITOR_DATA=~/tmp/data_clomonitor

alias clomonitor_db_init="mkdir -p $CLOMONITOR_DATA && initdb -U postgres $CLOMONITOR_DATA"
alias clomonitor_db_create="psql -U postgres -c 'create database clomonitor'"
alias clomonitor_db_create_tests="psql -U postgres -c 'create database clomonitor_tests' && psql -U postgres clomonitor_tests -c 'create extension if not exists pgtap'"
alias clomonitor_db_drop="psql -U postgres -c 'drop database clomonitor with (force)'"
alias clomonitor_db_drop_tests="psql -U postgres -c 'drop database if exists clomonitor_tests'"
alias clomonitor_db_recreate="clomonitor_db_drop && clomonitor_db_create && clomonitor_db_migrate"
alias clomonitor_db_recreate_tests="clomonitor_db_drop_tests && clomonitor_db_create_tests && clomonitor_db_migrate_tests"
alias clomonitor_db_server="postgres -D $CLOMONITOR_DATA"
alias clomonitor_db_client="psql -h localhost -U postgres clomonitor"
alias clomonitor_db_migrate="pushd $CLOMONITOR_SOURCE/database/migrations; TERN_CONF=~/.config/clomonitor/tern.conf ./migrate.sh; popd"
alias clomonitor_db_migrate_tests="pushd $CLOMONITOR_SOURCE/database/migrations; TERN_CONF=~/.config/clomonitor/tern-tests.conf ./migrate.sh; popd"
alias clomonitor_db_tests="pushd $CLOMONITOR_SOURCE/database/tests; pg_prove --host localhost --dbname clomonitor_tests --username postgres --verbose **/*.sql; popd"
alias clomonitor_apiserver="$CLOMONITOR_SOURCE/target/debug/clomonitor-apiserver -c ~/.config/clomonitor/apiserver.yaml"
alias clomonitor_tracker="$CLOMONITOR_SOURCE/target/debug/clomonitor-tracker -c ~/.config/clomonitor/tracker.yaml"
alias clomonitor_linter="$CLOMONITOR_SOURCE/target/debug/clomonitor-linter"
alias clomonitor_frontend_build="pushd $CLOMONITOR_SOURCE/web; yarn build; popd"
alias clomonitor_frontend_dev="pushd $CLOMONITOR_SOURCE/web; yarn start; popd"
alias clomonitor_frontend_tests="pushd $CLOMONITOR_SOURCE/web; yarn test; popd"
alias clomonitor_frontend_lint_fix="pushd $CLOMONITOR_SOURCE/web; yarn lint:fix; popd"
```
