use anyhow::Result;
use clap::{Parser, Subcommand};

mod pdfcrop;
mod pdfembed;
mod plain_photos;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    PdfEmbed(pdfembed::PdfEmbed),
    PdfCrop(pdfcrop::PdfCrop),
    PlainPhotos(plain_photos::PlainPhotos),
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::PdfCrop(opts) => {
            pdfcrop::run(opts.overwrite, &opts.files)?;
        }
        Commands::PdfEmbed(opts) => {
            pdfembed::run(opts.overwrite, opts.verify, &opts.files)?;
        }
        Commands::PlainPhotos(opts) => {
            plain_photos::run(&opts.basename, &opts.files)?;
        }
    }

    Ok(())
}
