use anyhow::Result;
use clap::{Parser, Subcommand};
use clap_verbosity_flag::{Verbosity, WarnLevel};

use tool::{gitemail, pdfcrop, pdfembed, plain_photos, semver, setup, wifiqr};

#[derive(Parser)]
#[command(author("Luke Hsiao"), version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    #[command(flatten)]
    verbose: Verbosity<WarnLevel>,
}

#[derive(Subcommand)]
enum Commands {
    GitEmail(gitemail::GitEmail),
    PdfCrop(pdfcrop::PdfCrop),
    PdfEmbed(pdfembed::PdfEmbed),
    PlainPhotos(plain_photos::PlainPhotos),
    Semver(semver::Semver),
    Setup(setup::Setup),
    WifiQR(wifiqr::WifiQR),
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    env_logger::Builder::new()
        .filter_level(cli.verbose.log_level_filter())
        .format_timestamp(None)
        .init();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::GitEmail(opts) => {
            gitemail::run(&opts.prefix, &opts.to)?;
        }
        Commands::PdfCrop(opts) => {
            pdfcrop::run(opts.overwrite, &opts.files)?;
        }
        Commands::PdfEmbed(opts) => {
            pdfembed::run(opts.overwrite, &opts.files)?;
        }
        Commands::PlainPhotos(opts) => {
            plain_photos::run(&opts.basename, &opts.files)?;
        }
        Commands::Semver(opts) => {
            semver::run(opts)?;
        }
        Commands::Setup(opts) => {
            setup::run(opts)?;
        }
        Commands::WifiQR(opts) => {
            wifiqr::run(opts)?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::*;
    #[test]
    fn verify_app() {
        use clap::CommandFactory;
        Cli::command().debug_assert()
    }
}
