use crate::linter::{
    CheckSet,
    check::{CheckId, CheckInput, CheckOutput},
};

use super::util::helpers::{find_exemption, should_skip_check};

/// Check identifier.
pub(crate) const ID: CheckId = "license_approved";

/// Check score weight.
pub(crate) const WEIGHT: usize = 10;

/// Check sets this check belongs to.
pub(crate) const CHECK_SETS: [CheckSet; 3] = [CheckSet::Code, CheckSet::CodeLite, CheckSet::Docs];

/// CNCF approved licenses.
/// https://github.com/cncf/foundation/blob/master/allowed-third-party-license-policy.md
static APPROVED_LICENSES: [&str; 11] = [
    "Apache-2.0",
    "BSD-2-Clause",
    "BSD-2-Clause-FreeBSD",
    "BSD-3-Clause",
    "CC-BY-4.0",
    "ISC",
    "MIT",
    "PostgreSQL",
    "Python-2.0",
    "X11",
    "Zlib",
];

/// Check main function.
pub(crate) fn check(input: &CheckInput, spdx_id: Option<String>) -> Option<CheckOutput> {
    // Check if this check should be skipped
    if should_skip_check(ID, &input.li.check_sets) {
        return None;
    }

    // Check if an exemption has been declared for this check
    if let Some(exemption) = find_exemption(ID, input.cm_md.as_ref()) {
        return Some(CheckOutput::from(exemption));
    }

    // SPDX id in list of approved licenses
    if spdx_id.is_some_and(|spdx_id| is_approved(&spdx_id)) {
        return Some(CheckOutput::passed());
    }

    Some(CheckOutput::not_passed())
}

/// Check if the license provided is an approved one.
fn is_approved(spdx_id: &str) -> bool {
    APPROVED_LICENSES.contains(&spdx_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn approved_license() {
        assert!(is_approved("Apache-2.0"));
        assert!(is_approved("MIT"));
    }

    #[test]
    fn non_approved_license() {
        assert!(!is_approved("AGPL-1.0-only"));
    }
}
