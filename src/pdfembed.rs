use std::path::PathBuf;

use anyhow::Result;
use clap::Args;
use xshell::{cmd, Shell};

#[derive(Args)]
/// Embed PDF fonts
pub(crate) struct PdfEmbed {
    #[clap(short, long)]
    /// Whether to overwrite the original files
    pub(crate) overwrite: bool,
    #[clap(short, long)]
    /// Whether to show pdffont output to verify the embedding
    pub(crate) verify: bool,
    #[clap(value_parser, min_values = 1, required = true)]
    /// The list of PDFs to embed fonts for
    pub(crate) files: Vec<PathBuf>,
}

pub(crate) fn run(overwrite: bool, verify: bool, files: &Vec<PathBuf>) -> Result<()> {
    let sh = Shell::new()?;

    // Embed fonts
    for file in files {
        let newfile = format!(
            "{}/emb_{}.{}",
            file.parent().unwrap().to_str().unwrap(),
            file.file_stem().unwrap().to_str().unwrap(),
            file.extension().unwrap().to_str().unwrap()
        );
        cmd!(sh, "pdftocairo -pdf {file} {newfile}").run()?;

        if overwrite {
            cmd!(sh, "mv {newfile} {file}").run()?;
        }

        if verify {
            cmd!(sh, "pdffonts {file}").run()?;
        }
    }

    Ok(())
}
