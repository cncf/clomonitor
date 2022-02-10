use anyhow::Error;
use askalono::*;
use glob::{glob_with, MatchOptions, PatternError};
use regex::RegexSet;
use std::fs;
use std::path::{Path, PathBuf};

/// SPDX license list. Used to detect license used by repositories.
const LICENSES: &[u8] = include_bytes!("data/licenses.bin.zstd");

/// CNCF approved licenses.
/// https://github.com/cncf/foundation/blob/master/allowed-third-party-license-policy.md
static APPROVED_LICENSES: [&str; 10] = [
    "Apache-2.0",
    "BSD-2-Clause",
    "BSD-2-Clause-FreeBSD",
    "BSD-3-Clause",
    "ISC",
    "MIT",
    "PostgreSQL",
    "Python-2.0",
    "X11",
    "Zlib",
];

/// Glob matching configuration.
#[derive(Debug)]
pub(crate) struct Globs<'a> {
    pub root: &'a Path,
    pub patterns: Vec<&'a str>,
    pub case_sensitive: bool,
}

/// Check if the content of any of the files that match the globs provided
/// matches any of the regular expressions given.
pub(crate) fn content_matches(globs: Globs, regexps: Vec<&str>) -> Result<bool, Error> {
    let re = RegexSet::new(regexps)?;
    Ok(matching_paths(globs)?.iter().any(|path| {
        if let Ok(content) = fs::read_to_string(path) {
            return re.is_match(&content);
        }
        false
    }))
}

/// Check repository's license and return its SPDX id if possible.
pub(crate) fn license(globs: Globs) -> Result<Option<String>, Error> {
    let store = Store::from_cache(LICENSES)?;
    let mut spdx_id: Option<String> = None;
    matching_paths(globs)?.iter().any(|path| {
        if let Ok(content) = fs::read_to_string(path) {
            let m = store.analyze(&TextData::from(content));
            if m.score > 0.9 {
                spdx_id = Some(m.name.to_string());
                return true;
            }
        }
        false
    });
    Ok(spdx_id)
}

/// Check if the license provided is an approved one.
pub(crate) fn is_approved_license(spdx_id: &str) -> bool {
    APPROVED_LICENSES.contains(&spdx_id)
}

/// Check if exists at least a path that matches the globs provided.
pub(crate) fn path_exists(globs: Globs) -> Result<bool, PatternError> {
    Ok(!matching_paths(globs)?.is_empty())
}

/// Return all paths that match any of the globs provided.
fn matching_paths(globs: Globs) -> Result<Vec<PathBuf>, PatternError> {
    let options = MatchOptions {
        case_sensitive: globs.case_sensitive,
        ..Default::default()
    };
    globs
        .patterns
        .iter()
        .map(|pattern| globs.root.join(pattern))
        .map(|pattern| pattern.to_string_lossy().into_owned())
        .try_fold(Vec::new(), |mut paths, pattern| {
            match glob_with(&pattern, options) {
                Ok(pattern_paths) => {
                    paths.extend(pattern_paths.filter_map(Result::ok));
                    Ok(paths)
                }
                Err(err) => Err(err),
            }
        })
}
