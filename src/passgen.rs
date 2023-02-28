use anyhow::Result;
use clap::Args;
use xshell::{cmd, Shell};

#[derive(Args)]
/// Generate a random password, like passwordstore.org
pub struct PassGen {
    #[arg(short, long)]
    /// If set, exlucde symbols from the password
    pub no_symbols: bool,
    /// How many charcters for the password
    pub length: u32,
}

pub fn run(opts: &PassGen) -> Result<()> {
    let sh = Shell::new()?;

    // From OWASP Password special character list
    // Ref: https://owasp.org/www-community/password-special-characters
    let char_set = r##"A-Za-z0-9!"#$%&'\''()*+,-./:;<=>?@[\]^_`{|}~"##;
    let char_set_no_symbols = "A-Za-z0-9";

    let cmd = if opts.no_symbols {
        format!(
            "LC_ALL=C tr -dc '{char_set_no_symbols}' < /dev/urandom | head -c {}",
            opts.length
        )
    } else {
        format!(
            "LC_ALL=C tr -dc '{char_set}' < /dev/urandom | head -c {}",
            opts.length
        )
    };

    let pass = cmd!(sh, "bash -c {cmd}").read()?;
    print!("{pass}");

    Ok(())
}
