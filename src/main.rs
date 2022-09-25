use anyhow::Result;
use clap::{Parser, Subcommand};
use clap_verbosity_flag::{Verbosity, WarnLevel};

mod gitemail;
mod pdfcrop;
mod pdfembed;
mod plain_photos;
mod semver;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
    #[clap(flatten)]
    verbose: Verbosity<WarnLevel>,
}

#[derive(Subcommand)]
enum Commands {
    GitEmail(gitemail::GitEmail),
    PdfEmbed(pdfembed::PdfEmbed),
    PdfCrop(pdfcrop::PdfCrop),
    PlainPhotos(plain_photos::PlainPhotos),
    Semver(semver::Semver),
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
            pdfembed::run(opts.overwrite, opts.verify, &opts.files)?;
        }
        Commands::PlainPhotos(opts) => {
            plain_photos::run(&opts.basename, &opts.files)?;
        }
        Commands::Semver(opts) => {
            semver::run(&opts.target)?;
        }
    }

    Ok(())
}
