use std::{fs::File, io::Write};

use anyhow::{Context, Result};
use clap::{Args, ValueEnum};
use log::debug;
use tempfile::tempdir;
use tera::Tera;
use xshell::{cmd, Shell};

use crate::Section;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum AuthType {
    WPA2,
}

impl AuthType {
    pub fn as_str(&self) -> &'static str {
        match self {
            AuthType::WPA2 => "WPA2",
        }
    }
}

#[derive(Args)]
/// Generate a PDF of a business-card-sized QR code for sharing WiFi credentials using LaTeX
pub struct WifiQR {
    #[arg(short, long)]
    /// The SSID of the wifi network.
    pub ssid: String,
    #[arg(short, long)]
    /// The password of the wifi network
    pub password: String,
    #[arg(short, long, value_enum, default_value_t = AuthType::WPA2)]
    /// The authentication type of the wifi network
    pub authtype: AuthType,
    #[arg(short, long, default_value = "")]
    /// The physical location of the network
    pub location: String,
}

pub fn run(opts: &WifiQR) -> Result<()> {
    let sh = Shell::new()?;
    let dir = tempdir()?;

    // TODO(lukehsiao): check that fonts / tectonic are installed.

    let _file = {
        let _s = Section::new("Populating LaTeX template");
        let template = include_str!("data/wifi.tex");
        let mut context = tera::Context::new();

        context.insert("password", &opts.password);
        context.insert("ssid", &opts.ssid);
        context.insert("authtype", opts.authtype.as_str());
        context.insert("location", &opts.location);
        let output = Tera::one_off(template, &context, true)
            .with_context(|| format!("Failed to parse Tera template:\n{}", template))?;
        debug!("{output}");

        let file_path = dir.path().join("wifi.tex");
        let mut file = File::create(file_path)?;
        writeln!(file, "{}", output)?;
        file
    };

    {
        let _s = Section::new("Running tectonic");
        let path = dir.path().join("wifi.tex");
        cmd!(sh, "tectonic {path}").run()?;
        let path = dir.path().join("wifi.pdf");
        cmd!(sh, "mv {path} .").run()?;
        println!("File written to ./wifi.pdf");
    }

    Ok(())
}
