use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};

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
    /// Strip all Exif data from photos and rename as <basename>_{:04}.<ext>
    PlainPhotos {
        #[clap(short, long, value_parser, required = true)]
        /// The basename to rename all files to
        basename: String,
        #[clap(value_parser, min_values = 1, required = true)]
        /// The list of images to string Exif data from and rename
        files: Vec<PathBuf>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::PlainPhotos { basename, files } => {
            plain_photos::run(basename, files)?;
        }
    }

    Ok(())
}
