use anyhow::Result;
use clap::{Args, ValueEnum};
use tempfile::tempdir;
use xshell::{Shell, cmd};

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
/// Generate a PDF of a business-card-sized QR code for sharing WiFi credentials using Typst
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

    // TODO(lukehsiao): check that fonts / typst are installed.

    {
        let _s = Section::new("Writing Typst sources");
        std::fs::write(dir.path().join("wifi.typ"), include_str!("data/wifi.typ"))?;
        std::fs::write(
            dir.path().join("wifi.svg"),
            include_str!("data/lucide-wifi.svg"),
        )?;
        std::fs::write(
            dir.path().join("key.svg"),
            include_str!("data/lucide-key-round.svg"),
        )?;
    }

    {
        let _s = Section::new("Running typst");
        let typ = dir.path().join("wifi.typ");
        let root = dir.path();
        // Credentials are passed as `sys.inputs` rather than substituted into the
        // template, so values that look like Typst markup cannot be injected.
        let ssid_in = format!("ssid={}", opts.ssid);
        let pass_in = format!("password={}", opts.password);
        let auth_in = format!("authtype={}", opts.authtype.as_str());
        let loc_in = format!("location={}", opts.location);
        cmd!(
            sh,
            "typst compile --root {root} --input {ssid_in} --input {pass_in} --input {auth_in} --input {loc_in} {typ} wifi.pdf"
        )
        .run()?;
        println!("File written to ./wifi.pdf");
    }

    Ok(())
}
