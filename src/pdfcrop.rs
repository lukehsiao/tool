use std::path::PathBuf;

use anyhow::Result;
use clap::Args;
use xshell::{cmd, Shell};

#[derive(Args)]
/// Crop PDF file to just the contents
pub struct PdfCrop {
    #[arg(short, long)]
    /// If false, a new file will be created with the `crop_` prefix
    pub overwrite: bool,
    #[arg(num_args(1..), required = true)]
    /// The list of PDFs to crop
    pub files: Vec<PathBuf>,
}

pub fn run(overwrite: bool, files: &[PathBuf]) -> Result<()> {
    let sh = Shell::new()?;

    // Embed fonts
    for file in files {
        let mut newfile = PathBuf::from(file);
        newfile.set_file_name(format!(
            "crop_{}.{}",
            file.file_stem().unwrap().to_str().unwrap(),
            file.extension().unwrap().to_str().unwrap()
        ));
        cmd!(sh, "pdfcrop --hires {file} {newfile}").run()?;

        if overwrite {
            cmd!(sh, "mv {newfile} {file}").run()?;
        }
    }

    Ok(())
}
