use std::io::Write;

use anyhow::{bail, Result};
use clap::Args;
use log::warn;
use semver::Version;
use tempfile::NamedTempFile;
use xshell::{cmd, Shell};

#[derive(Args)]
/// Script to help cut a new release of a project.
///
/// Makes sure things are up-to-date, calls the repository-specific `contrib/_incr_version` script,
/// then creates a new annotated git tag auto-populated with the shortlog. Note this calls
/// `_incr_version` with two parameters, the old version, and the new version.
///
/// Refs: https://drewdevault.com/2019/10/12/how-to-fuck-up-releases.html
pub(crate) struct Semver {
    #[clap(arg_enum, value_parser)]
    /// How to bump the version number (major/minor/patch), or bump to a specific version (e.g.,
    /// "v0.1.1")
    pub(crate) target: String,
}

pub(crate) fn run(target: &str) -> Result<()> {
    let sh = Shell::new()?;

    // Make sure we're on the right branch
    let current_branch = cmd!(sh, "git branch --show-current").read()?;
    let origin_head = duct::cmd!("git", "remote", "show", "origin")
        .pipe(duct::cmd!("grep", "HEAD branch:"))
        .pipe(duct::cmd!("awk", "{print $3}"))
        .read()?;
    if current_branch != origin_head {
        warn!("Not on {}.", origin_head)
    }

    // Make sure we're up-to-date
    cmd!(sh, "git pull --rebase").run()?;

    // Get latest tag
    let latest_tag = match cmd!(sh, "git describe --abbrev=0").read() {
        Ok(tag) => tag,
        Err(_) => {
            warn!("No tags present in repository. Defaulting to v0.0.1.");
            "v0.0.0".to_string()
        }
    };
    let latest_version = Version::parse(&latest_tag[1..])?;

    // Compute next version
    let next_version = match target {
        "major" => Version::new(latest_version.major + 1, 0, 0),
        "minor" => Version::new(latest_version.major, latest_version.minor + 1, 0),
        "patch" => Version::new(
            latest_version.major,
            latest_version.minor,
            latest_version.patch + 1,
        ),
        t if t.starts_with('v') => Version::parse(&t[1..])?,
        _ => {
            bail!(
                "User-specified version should be of the form vX.X.X: {}",
                target
            );
        }
    };
    let next_version = format!("v{}", next_version.to_string());

    // Grab shortlog
    let toplevel = cmd!(sh, "git rev-parse --show-toplevel").read()?;
    let reponame = cmd!(sh, "basename {toplevel}").read()?;
    let shortlog = match latest_tag.as_str() {
        "v0.0.0" => cmd!(sh, "git shortlog -e --no-merges HEAD").read()?,
        tag => cmd!(sh, "git shortlog -e --no-merges {tag}..HEAD").read()?,
    };
    let mut shortlog_file = NamedTempFile::new()?;
    writeln!(shortlog_file, "{reponame} {}\n\n{}", next_version, shortlog)?;

    // Run contrib/_incr_version
    match cmd!(sh, "contrib/_incr_version {latest_tag} {next_version}").run() {
        Ok(_) => (),
        Err(e) => {
            bail!(
                "{}.\nDoes this project have any specific release requirements?",
                e
            );
        }
    }

    // Create a new git tag
    let path = shortlog_file.path().to_str().unwrap();
    cmd!(sh, "git tag -e -F {path} -a {next_version}").run()?;

    Ok(())
}
