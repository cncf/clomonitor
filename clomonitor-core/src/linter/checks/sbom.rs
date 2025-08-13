use std::sync::LazyLock;

use anyhow::Result;
use regex::RegexSet;

use crate::linter::{
    CheckSet,
    check::{CheckId, CheckInput, CheckOutput},
};

use super::{datasource::github, util::helpers::readme_matches};

/// Check identifier.
pub(crate) const ID: CheckId = "sbom";

/// Check score weight.
pub(crate) const WEIGHT: usize = 1;

/// Check sets this check belongs to.
pub(crate) const CHECK_SETS: [CheckSet; 1] = [CheckSet::Code];

static README_REF: LazyLock<RegexSet> = LazyLock::new(|| {
    RegexSet::new([
        r"(?im)^#+.*sbom.*$",
        r"(?im)^#+.*software bill of materials.*$",
        r"(?im)^sbom$",
        r"(?im)^software bill of materials$",
    ])
    .expect("exprs in README_REF to be valid")
});

static RELEASE_REF: LazyLock<RegexSet> =
    LazyLock::new(|| RegexSet::new([r"(?i)sbom"]).expect("exprs in RELEASE_REF to be valid"));

/// Check main function.
pub(crate) fn check(input: &CheckInput) -> Result<CheckOutput> {
    // Asset in last release
    if let Some(true) = github::latest_release(&input.gh_md)
        .and_then(|r| r.release_assets.nodes.as_ref())
        .map(|assets| {
            assets
                .iter()
                .flatten()
                .any(|asset| RELEASE_REF.is_match(&asset.name))
        })
    {
        return Ok(CheckOutput::passed());
    }

    // Reference in README file
    if readme_matches(&input.li.root, &README_REF)? {
        return Ok(CheckOutput::passed());
    }

    Ok(CheckOutput::not_passed())
}

#[cfg(test)]
mod tests {
    use anyhow::format_err;

    use crate::linter::{
        LinterInput,
        datasource::github::md::{
            MdRepository, MdRepositoryReleases, MdRepositoryReleasesNodes,
            MdRepositoryReleasesNodesReleaseAssets, MdRepositoryReleasesNodesReleaseAssetsNodes,
        },
    };

    use super::*;

    #[test]
    fn not_passed_no_release_found() {
        assert_eq!(
            check(&CheckInput {
                li: &LinterInput::default(),
                cm_md: None,
                gh_md: MdRepository {
                    ..MdRepository::default()
                },
                scorecard: Err(format_err!("no scorecard available")),
                security_insights: Ok(None),
            })
            .unwrap(),
            CheckOutput::not_passed(),
        );
    }

    #[test]
    fn not_passed_no_ref_in_release_found() {
        assert_eq!(
            check(&CheckInput {
                li: &LinterInput::default(),
                cm_md: None,
                gh_md: MdRepository {
                    releases: MdRepositoryReleases {
                        nodes: Some(vec![Some(MdRepositoryReleasesNodes {
                            created_at: "created_at_date".to_string(),
                            description: None,
                            is_latest: true,
                            is_prerelease: false,
                            release_assets: MdRepositoryReleasesNodesReleaseAssets {
                                nodes: Some(vec![Some(
                                    MdRepositoryReleasesNodesReleaseAssetsNodes {
                                        name: "test.txt".to_string()
                                    }
                                )])
                            },
                            url: "release_url".to_string(),
                        })]),
                    },
                    ..MdRepository::default()
                },
                scorecard: Err(format_err!("no scorecard available")),
                security_insights: Ok(None),
            })
            .unwrap(),
            CheckOutput::not_passed(),
        );
    }

    #[test]
    fn passed_ref_found_in_latest_release() {
        assert_eq!(
            check(&CheckInput {
                li: &LinterInput::default(),
                cm_md: None,
                gh_md: MdRepository {
                    releases: MdRepositoryReleases {
                        nodes: Some(vec![Some(MdRepositoryReleasesNodes {
                            created_at: "created_at_date".to_string(),
                            description: None,
                            is_latest: true,
                            is_prerelease: false,
                            release_assets: MdRepositoryReleasesNodesReleaseAssets {
                                nodes: Some(vec![Some(
                                    MdRepositoryReleasesNodesReleaseAssetsNodes {
                                        name: "test_sbom.spdx.json".to_string()
                                    }
                                )])
                            },
                            url: "release_url".to_string(),
                        })]),
                    },
                    ..MdRepository::default()
                },
                scorecard: Err(format_err!("no scorecard available")),
                security_insights: Ok(None),
            })
            .unwrap(),
            CheckOutput::passed(),
        );
    }

    #[test]
    fn readme_ref_match() {
        assert!(README_REF.is_match("# SBOM"));
        assert!(README_REF.is_match("# Software Bill of Materials"));
        assert!(README_REF.is_match(
            r"
...
## Project SBOM
...
            "
        ));
        assert!(README_REF.is_match(
            r"
...
Software Bill of Materials
--------------------------
...
            "
        ));
    }

    #[test]
    fn release_ref_match() {
        assert!(RELEASE_REF.is_match("test_sbom.spdx.json"));
    }
}
