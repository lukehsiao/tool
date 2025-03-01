use std::path::{Path, PathBuf};

use anyhow::{Result, bail};
use clap::Args;
use xshell::{Shell, cmd};

#[derive(Args)]
/// Encode the input video using 2-pass constant-quality VP9 to {output}.webm.
pub struct Vp9 {
    #[arg(short, long, required = true)]
    /// The input file
    pub input: PathBuf,
    #[arg(short, long, default_value_t = 30)]
    /// The crf value to use
    pub crf: u8,
    #[arg(short('y'))]
    /// whether to overwrite the output file if it exists
    pub overwrite: bool,
    #[arg(short, long, required = true)]
    /// The name to give the output file
    pub output: String,
}

pub fn run(opts: &Vp9) -> Result<()> {
    let sh = Shell::new()?;

    // Quick check if file exists to not waste 1-pass time
    if !opts.overwrite && Path::new(&format!("{}.webm", opts.output)).exists() {
        bail!("The output file already exists. Please use `-y` if you want to overwrite.")
    }

    let cmd = format!(
        "ffmpeg -i '{}' -c:v libvpx-vp9 -row-mt 1 -b:v 0 -crf {} -pass 1 -an -f null /dev/null && \
ffmpeg {} -i '{}' -c:v libvpx-vp9 -row-mt 1 -b:v 0 -crf {} -pass 2 -c:a libopus '{}.webm'",
        opts.input.display(),
        opts.crf,
        { if opts.overwrite { "-y" } else { "-n" } },
        opts.input.display(),
        opts.crf,
        opts.output
    );

    cmd!(sh, "bash -c {cmd}").read()?;
    Ok(())
}
