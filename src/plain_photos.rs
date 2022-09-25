use std::path::PathBuf;

use anyhow::Result;
use xshell::{cmd, Shell};

pub(crate) fn run(basename: &str, files: &Vec<PathBuf>) -> Result<()> {
    let sh = Shell::new()?;
    // Strip all exif data using `exiv2`.
    cmd!(sh, "exiv2 -d a -k {files...}").run()?;

    // Rename files
    // TODO(lukehsiao): kind of weird to do this with `mv`...
    for (i, file) in files.iter().enumerate() {
        let new = format!(
            "{}/{basename}_{:04}.{}",
            file.as_path().parent().unwrap().to_str().unwrap(),
            i,
            file.as_path().extension().unwrap().to_str().unwrap()
        );
        cmd!(sh, "mv {file} {new}").run()?;
    }

    Ok(())
}
