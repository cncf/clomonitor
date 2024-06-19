# New check

Adding a new check to CLOMonitor involves a number of steps. This document tries to summarize the process by providing some information about each of those steps.

*NOTE: if you are unsure if the new check is aligned with CLOMonitor's goals you may want to file an issue first. The issue can detail the new check and you can get feedback from the maintainers prior to starting to work on it.*

## Steps to add a new check

### 1. Create a new file for the check in the checks directory

The check's file, which must be located in [clomonitor-core/src/linter/checks](https://github.com/cncf/clomonitor/tree/main/clomonitor-core/src/linter/checks), must declare the following information:

* `ID`: check identifier
* `WEIGHT`: weight of this check, used to calculate scores
* `CHECK_SETS`: check sets this new check belongs to

The **entrypoint** for the check must be a function named `check`, with the following signature:

* Sync check: `pub(crate) fn check(input: &CheckInput) -> Result<CheckOutput<T>>`
* Async check: `pub(crate) async fn check(input: &CheckInput<'_>) -> Result<CheckOutput<T>>`

In the [clomonitor-core/src/linter/checks/util](https://github.com/cncf/clomonitor/tree/main/clomonitor-core/src/linter/checks/util) directory, there are some helpers that can be useful when writing new checks.

### 2. Register the new check

The new check must be registered in the [checks module](https://github.com/cncf/clomonitor/blob/main/clomonitor-core/src/linter/checks/mod.rs) file: its module must be declared and the `register_check!` macro called, passing to it the module name.

### 3. Extend the report with the new check

A field for the new check must be added to the corresponding report section structure (*documentation*, *license*, *best practices*, *security* or *legal*) in the [clomonitor-core/src/linter/report.rs](https://github.com/cncf/clomonitor/blob/main/clomonitor-core/src/linter/report.rs) file. The new check's module should also be added to the corresponding `section_impl!` macro call in the same file.

The report struct is used in a few places across the codebase, so after adding the new check's field we'll need to include it in a few places, including some tests (`cargo check` may be of help guiding you in this process). One of those places will be the [clomonitor-core/src/linter/mod.rs](https://github.com/cncf/clomonitor/blob/main/clomonitor-core/src/linter/mod.rs), where the new check is called when building the report.

### 4. Add the new check to the linter CLI tool

The linter CLI tools supports displaying a report as a table. When adding a new check, a new row must be added to the [output table](https://github.com/cncf/clomonitor/blob/main/clomonitor-linter/src/table.rs) and the [display.golden test file](https://github.com/cncf/clomonitor/blob/main/clomonitor-linter/src/testdata/display.golden) must be updated accordingly.

### 5. Add the new check to the report's markdown version

The report's markdown version is generated from a [template](https://github.com/cncf/clomonitor/blob/main/clomonitor-apiserver/templates/repository-report.md) that needs to be updated with the new check.

### 6. Update database functions

The following database functions (and their corresponding tests) must be updated to include the new check:

* [get_repositories_with_checks.sql](https://github.com/cncf/clomonitor/blob/main/database/migrations/functions/repositories/get_repositories_with_checks.sql)
* [get_stats.sql](https://github.com/cncf/clomonitor/blob/main/database/migrations/functions/stats/get_stats.sql)

### 7. Prepare UI to display the new check

The new check must also be registered in the UI so that we can display it in reports and other views. This involves:

* Register it in the [web/src/types.ts](https://github.com/cncf/clomonitor/blob/main/web/src/types.ts) file (in `ReportOption`).
* Define some information about the check and include it in the corresponding section in `CHECKS_PER_CATEGORY` in the [web/src/data.tsx](https://github.com/cncf/clomonitor/blob/main/web/src/data.tsx) file. This includes picking up an icon for it, which should come from [React Icons](https://react-icons.github.io/react-icons).

### 8. Document the new check

The new check must be documented in the [checks.md](https://github.com/cncf/clomonitor/blob/main/docs/checks.md) file. This includes adding a new entry in the [documentation section](https://github.com/cncf/clomonitor/blob/main/docs/checks.md#documentation) of the file, as well as listing the new check in the corresponding check sets.
