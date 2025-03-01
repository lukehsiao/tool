use std::path::PathBuf;

use anyhow::Result;
use clap::Args;
use xshell::{Shell, cmd};

#[derive(Args)]
/// Embed PDF fonts
pub struct PdfEmbed {
    #[arg(short, long)]
    /// If false, a new file will be created with the `emb_` prefix
    pub overwrite: bool,
    #[arg(num_args(1..), required = true)]
    /// The list of PDFs to embed fonts for
    pub files: Vec<PathBuf>,
}

pub fn run(overwrite: bool, files: &[PathBuf]) -> Result<()> {
    let sh = Shell::new()?;

    // Embed fonts
    for file in files {
        let mut newfile = PathBuf::from(file);
        newfile.set_file_name(format!(
            "emb_{}.{}",
            file.file_stem().unwrap().to_str().unwrap(),
            file.extension().unwrap().to_str().unwrap()
        ));
        cmd!(sh, "pdftocairo -pdf {file} {newfile}").run()?;

        if overwrite {
            cmd!(sh, "mv {newfile} {file}").run()?;
            cmd!(sh, "pdffonts {file}").run()?;
        } else {
            cmd!(sh, "pdffonts {newfile}").run()?;
        }
    }

    Ok(())
}
