use std::{fs, path::Path};

use anyhow::{format_err, Result};

/// Wrapper around fs::read_to_string that does not follow symlinks.
pub(crate) fn read_to_string<P: AsRef<Path>>(path: P) -> Result<String> {
    // Return an error if the path provided is a symlink
    let attr = fs::symlink_metadata(&path)?;
    if attr.is_symlink() {
        return Err(format_err!("symlinks not supported"));
    }

    fs::read_to_string(&path).map_err(|e| format_err!("{e:?}"))
}
