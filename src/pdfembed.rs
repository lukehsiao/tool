use std::path::PathBuf;

use anyhow::Result;
use xshell::{cmd, Shell};

/// Embed PDF fonts
pub(crate) fn run(overwrite: &bool, verify: &bool, files: &Vec<PathBuf>) -> Result<()> {
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

        if *overwrite {
            cmd!(sh, "mv {newfile} {file}").run()?;
        }

        if *verify {
            cmd!(sh, "pdffonts {file}").run()?;
        }
    }

    Ok(())
}
