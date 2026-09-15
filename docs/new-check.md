# New check

Adding a new check to CLOMonitor involves updating each surface that defines,
runs, scores, displays, stores, filters, and documents check results.

*NOTE: if you are unsure if the new check is aligned with CLOMonitor's goals
you may want to file an issue first. The issue can detail the new check and you
can get feedback from the maintainers prior to starting to work on it.*

## Steps to add a new check

### 1. Create a new check module

The check's file must be located in
[clomonitor-core/src/linter/checks](https://github.com/cncf/clomonitor/tree/main/clomonitor-core/src/linter/checks).
It declares the following information:

- `ID`: check identifier.
- `WEIGHT`: weight of this check, used to calculate scores.
- `CHECK_SETS`: check sets this check belongs to.

The **entrypoint** for the check must be a function named `check`, with one of
the following signatures:

- Sync check: `pub(crate) fn check(input: &CheckInput) -> Result<CheckOutput<T>>`
- Async check:
  `pub(crate) async fn check(input: &CheckInput<'_>) -> Result<CheckOutput<T>>`

Helpers for implementing checks are available in
[clomonitor-core/src/linter/checks/util](https://github.com/cncf/clomonitor/tree/main/clomonitor-core/src/linter/checks/util).

### 2. Register the check and datasource

The check module must be declared in
[clomonitor-core/src/linter/checks/mod.rs](https://github.com/cncf/clomonitor/blob/main/clomonitor-core/src/linter/checks/mod.rs).
The `register_check!` macro registers the module and, when needed, the external
datasource used by the check:

- CLOMonitor-only check: `register_check!(module_name)`
- OpenSSF Scorecard-backed check:
  `register_check!(module_name, scorecard = "Scorecard-Check-Name")`
- AFDocs-backed check:
  `register_check!(module_name, afdocs = "afdocs-category-id")`

### 3. Extend the report section

A field for the check must be added to the corresponding report section
structure in
[clomonitor-core/src/linter/report.rs](https://github.com/cncf/clomonitor/blob/main/clomonitor-core/src/linter/report.rs).
The check's module must also be listed in the matching `section_impl!` macro.

The linter report is built in
[clomonitor-core/src/linter/mod.rs](https://github.com/cncf/clomonitor/blob/main/clomonitor-core/src/linter/mod.rs),
so the new check must be called there when the report section is assembled.

### 4. Update scoring coverage

Scores are calculated in
[clomonitor-core/src/score/mod.rs](https://github.com/cncf/clomonitor/blob/main/clomonitor-core/src/score/mod.rs).
The score tests in that module should cover the new check's weight, pass/fail
behavior, missing-check behavior, merged scores, and any affected section or
global score expectations.

### 5. Add the check to the linter CLI output

The linter CLI displays reports as a table. The check must be added to
[clomonitor-linter/src/table.rs](https://github.com/cncf/clomonitor/blob/main/clomonitor-linter/src/table.rs),
and the
[display.golden](https://github.com/cncf/clomonitor/blob/main/clomonitor-linter/src/testdata/display.golden)
test fixture must match the rendered output.

### 6. Add the check to generated reports

The API server renders markdown and visual report summaries. Check-level
changes require updating:

- [clomonitor-apiserver/templates/repository-report.md](https://github.com/cncf/clomonitor/blob/main/clomonitor-apiserver/templates/repository-report.md)
- The corresponding repository report golden fixture.

Section-level changes also require updating:

- [clomonitor-apiserver/templates/report-summary.svg](https://github.com/cncf/clomonitor/blob/main/clomonitor-apiserver/templates/report-summary.svg)
- The corresponding report summary SVG golden fixture.

### 7. Update database functions

The database stores reports as JSONB and exposes projected check information for
CSV exports, statistics, and search filters. The following database functions
and their pgTAP tests must be kept in sync with the new check:

- [database/migrations/functions/repositories/get_repositories_with_checks.sql](https://github.com/cncf/clomonitor/blob/main/database/migrations/functions/repositories/get_repositories_with_checks.sql)
- [database/migrations/functions/stats/get_stats.sql](https://github.com/cncf/clomonitor/blob/main/database/migrations/functions/stats/get_stats.sql)
- [database/tests/functions](https://github.com/cncf/clomonitor/tree/main/database/tests/functions)

When a check identifier participates in project `passed_checks`, pgTAP coverage
should also prove that project check aggregation and `passed_checks` filtering
work with the identifier.

### 8. Prepare the UI to display the check

The web application needs type and display metadata for each check. Update:

- [web/src/types.ts](https://github.com/cncf/clomonitor/blob/main/web/src/types.ts):
  `ReportOption`.
- [web/src/data.tsx](https://github.com/cncf/clomonitor/blob/main/web/src/data.tsx):
  `SECTIONS`, `REPORT_OPTIONS`, and `CHECKS_PER_CATEGORY`.
- [web/src/data.test.tsx](https://github.com/cncf/clomonitor/blob/main/web/src/data.test.tsx):
  coverage for section/check metadata and category membership.

The check metadata includes its display name, optional short name, icon,
reference URL, and category membership.

### 9. Document the check

The check must be documented in
[docs/checks.md](https://github.com/cncf/clomonitor/blob/main/docs/checks.md).
The documentation includes the check set list entry, check identifier, behavior,
and external datasource reference when applicable.

## Adding a new section

A new report section requires all check-level touch points plus the section
contract in every scoring and display surface:

- Add the report section structure in
  [clomonitor-core/src/linter/report.rs](https://github.com/cncf/clomonitor/blob/main/clomonitor-core/src/linter/report.rs),
  include it in `Report` with `#[serde(default)]` for legacy stored reports, and
  register its checks with `section_impl!`.
- Add `Score` fields for the section score and section weight in
  [clomonitor-core/src/score/mod.rs](https://github.com/cncf/clomonitor/blob/main/clomonitor-core/src/score/mod.rs).
- Add the section to the `SECTIONS` table in `score/mod.rs` and set
  `counts_toward_global` to match the intended global-score semantics.
- Extend SQL statistics averages in
  [database/migrations/functions/stats/get_stats.sql](https://github.com/cncf/clomonitor/blob/main/database/migrations/functions/stats/get_stats.sql)
  and the corresponding pgTAP expectations.
- Add the section row to `report-summary.svg`, adjust the SVG height and
  positioning, and update the report-summary golden fixture.
- Add the section to the markdown repository report template and golden
  fixture.
- Add the section to the frontend `SECTIONS` metadata so summaries, filters,
  detail views, and statistics can render it consistently.
