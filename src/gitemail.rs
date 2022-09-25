use anyhow::Result;
use clap::{builder::ValueHint, Args};
use xshell::{cmd, Shell};

#[derive(Args)]
/// Set the format.subjectprefix and sendemail.to of the local repository.
pub(crate) struct GitEmail {
    #[clap(short, long)]
    /// Set the local repo's patch subject prefix
    pub(crate) prefix: Option<String>,
    #[clap(short, long, value_parser, min_values = 1, required = true, value_hint = ValueHint::EmailAddress)]
    /// List of emails to send to for `git-send-email`
    pub(crate) to: Vec<String>,
}

pub(crate) fn run(prefix: &Option<String>, to: &[String]) -> Result<()> {
    let sh = Shell::new()?;

    if let Some(p) = prefix {
        let prefix = format!("PATCH {p}");
        cmd!(sh, "git config format.subjectprefix {prefix}").run()?;
    }

    let emails = to.join(",");

    cmd!(sh, "git config sendemail.to {emails}").run()?;

    Ok(())
}
