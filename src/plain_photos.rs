use std::path::PathBuf;

use anyhow::Result;
use clap::Args;
use xshell::{cmd, Shell};

#[derive(Args)]
/// Strip all Exif data from photos and rename as <basename>_{:04}.<ext>
pub(crate) struct PlainPhotos {
    #[clap(short, long, value_parser, required = true)]
    /// The basename to rename all files to
    pub(crate) basename: String,
    #[clap(value_parser, min_values = 1, required = true)]
    /// The list of images to string Exif data from and rename
    pub(crate) files: Vec<PathBuf>,
}

pub(crate) fn run(basename: &str, files: &Vec<PathBuf>) -> Result<()> {
    let sh = Shell::new()?;
    // Strip all exif data using `exiv2`.
    cmd!(sh, "exiv2 -d a -k {files...}").run()?;

    // Rename files
    // TODO(lukehsiao): kind of weird to do this with `mv`...
    for (i, file) in files.iter().enumerate() {
        let new = format!(
            "{}/{basename}_{:04}.{}",
            file.parent().unwrap().to_str().unwrap(),
            i,
            file.extension().unwrap().to_str().unwrap()
        );
        cmd!(sh, "mv {file} {new}").run()?;
    }

    Ok(())
}
