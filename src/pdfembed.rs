use std::path::PathBuf;

use anyhow::Result;
use clap::Args;
use xshell::{cmd, Shell};

#[derive(Args)]
/// Embed PDF fonts
pub(crate) struct PdfEmbed {
    #[clap(short, long)]
    /// If false, a new file will be created with the `emb_` prefix
    pub(crate) overwrite: bool,
    #[clap(short, long)]
    /// Whether to show pdffont output to verify the embedding
    pub(crate) verify: bool,
    #[clap(value_parser, min_values = 1, required = true)]
    /// The list of PDFs to embed fonts for
    pub(crate) files: Vec<PathBuf>,
}

pub(crate) fn run(overwrite: bool, verify: bool, files: &[PathBuf]) -> Result<()> {
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

        match (overwrite, verify) {
            (true, true) => {
                cmd!(sh, "mv {newfile} {file}").run()?;
                cmd!(sh, "pdffonts {file}").run()?;
            }
            (true, false) => {
                cmd!(sh, "mv {newfile} {file}").run()?;
            }
            (false, true) => {
                cmd!(sh, "pdffonts {newfile}").run()?;
            }
            (false, false) => {}
        }
    }

    Ok(())
}
