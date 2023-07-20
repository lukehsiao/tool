use std::path::PathBuf;

use anyhow::Result;
use clap::Args;
use xshell::{cmd, Shell};

#[derive(Args)]
/// Strip all Exif data from photos and rename as <basename>_{:04}.<ext>
pub struct PlainPhotos {
    #[arg(short, long, required = true)]
    /// The basename to rename all files to
    pub basename: String,
    #[arg(num_args(1..), required = true)]
    /// The list of images to string Exif data from and rename
    pub files: Vec<PathBuf>,
}

pub fn run(basename: &str, files: &[PathBuf]) -> Result<()> {
    let sh = Shell::new()?;
    // Strip all metadata using `mat2`.
    // Ref: https://0xacab.org/jvoisin/mat2
    cmd!(sh, "mat2 --inplace {files...}").run()?;

    // Rename files
    // TODO(lukehsiao): kind of weird to do this with `mv`...
    for (i, file) in files.iter().enumerate() {
        let mut newfile = PathBuf::from(file);
        newfile.pop();
        newfile.push(format!(
            "{basename}_{:04}.{}",
            i,
            file.extension().unwrap().to_str().unwrap()
        ));
        cmd!(sh, "mv {file} {newfile}").run()?;
    }

    Ok(())
}
