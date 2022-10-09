use anyhow::Result;
use clap::{builder::ValueHint, Args};
use xshell::{cmd, Shell};

#[derive(Args)]
/// Set the format.subjectprefix and sendemail.to of the local repository.
pub struct GitEmail {
    #[arg(short, long)]
    /// Set the local repo's patch subject prefix. Defaults to repo name.
    pub prefix: Option<String>,
    #[arg(short, long, num_args(1..), required = true, value_hint = ValueHint::EmailAddress)]
    /// List of comma-separated emails to send to for `git-send-email`
    pub to: Vec<String>,
}

pub fn run(prefix: &Option<String>, to: &[String]) -> Result<()> {
    let sh = Shell::new()?;

    if let Some(p) = prefix {
        let prefix = format!("PATCH {p}");
        cmd!(sh, "git config format.subjectprefix {prefix}").run()?;
    } else {
        let toplevel = cmd!(sh, "git rev-parse --show-toplevel").read()?;
        let reponame = cmd!(sh, "basename {toplevel}").read()?;
        let prefix = format!("PATCH {reponame}");
        cmd!(sh, "git config format.subjectprefix {prefix}").run()?;
    }

    let emails = to.join(",");

    cmd!(sh, "git config sendemail.to {emails}").run()?;

    Ok(())
}
