use lazy_static::lazy_static;
use regex::Regex;
use std::path::Path;

/// Maximum number of commits used to check if the repository requires DCO.
pub const DCO_MAX_COMMITS: usize = 20;

/// Check if the last commits on the git repository located in the path
/// provided have the DCO signature.
pub(crate) fn commits_have_dco_signature(path: &Path) -> Result<bool, git2::Error> {
    lazy_static! {
        static ref MERGE_PR_RE: Regex = Regex::new(r"^Merge pull request ").unwrap();
        static ref MERGE_BRANCH_RE: Regex = Regex::new(r"^Merge branch ").unwrap();
        static ref DCO_SIGNATURE_RE: Regex = Regex::new(r"(?m)^Signed-off-by: ").unwrap();
    }

    let repo = git2::Repository::open(path)?;
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;

    let (mut processed, mut signed_off, mut merge) = (0, 0, 0);
    for oid in revwalk.take(DCO_MAX_COMMITS) {
        if oid.is_err() {
            continue;
        }
        let commit = repo.find_commit(oid.unwrap())?;
        processed += 1;
        if let Some(msg) = commit.message() {
            if MERGE_PR_RE.is_match(msg) || MERGE_BRANCH_RE.is_match(msg) {
                merge += 1;
                continue;
            }
            if DCO_SIGNATURE_RE.is_match(msg) {
                signed_off += 1;
            } else {
                return Ok(false);
            }
        }
    }

    if signed_off == processed - merge {
        return Ok(true);
    }
    Ok(false)
}
