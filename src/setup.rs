use anyhow::{ensure, Result};
use clap::Args;
use log::warn;
use regex::Regex;
use xshell::{cmd, Shell};

use crate::{regex, Section};

/// A list of commonly-used packages.
const APT_PACKAGES: &[&str] = &[
    "apt-transport-https",
    "asciinema",
    "autoconf",
    "automake",
    "build-essential",
    "checkinstall",
    "cmake",
    "curl",
    "dict",
    "exuberant-ctags",
    "finger",
    "firefox",
    "gir1.2-glib-2.0",
    "git-email",
    "git-lfs",
    "git-sizer",
    "gnupg2",
    "htop",
    "imagemagick",
    "iperf3",
    "jq",
    "lcov",
    "libbz2-dev",
    "libclang-dev",
    "libffi-dev",
    "liblzma-dev",
    "libmagickwand-dev",
    "libncurses5-dev",
    "libncursesw5-dev",
    "libpq-dev",
    "libreadline-dev",
    "libsqlite3-dev",
    "libssl-dev",
    "libxslt1-dev",
    "llvm",
    "m4",
    "make",
    "mdadm",
    "neofetch",
    "net-tools",
    "npm",
    "pass",
    "postgresql-client",
    "ppa-purge",
    "prettyping",
    "pv",
    "tmux",
    "units",
    "valgrind",
    "wget",
    "xz-utils",
    "zlib1g-dev",
    "zstd",
];

#[derive(Args)]
/// Script to install "core" system software I commonly use.
pub struct Setup {}

pub fn run(_opts: &Setup) -> Result<()> {
    let sh = Shell::new()?;
    {
        let _s = Section::new("Checking system OS");
        ensure!(
            (std::env::consts::OS == "linux"
                && cmd!(sh, "lsb_release -is").read()? == "Pop"
                && cmd!(sh, "lsb_release -rs").read()? == "22.04"),
            "This script is only designed for Pop!_OS 22.04"
        );
    }
    {
        let _s = Section::new("Special PPAs and pre-reqs");
        // Install Caddy Server
        // Ref: https://caddyserver.com/docs/install#debian-ubuntu-raspbian
        let bash = r#"
            sudo apt-get install -y debian-keyring debian-archive-keyring apt-transport-https &&
            curl -1sLf 'https://dl.cloudsmith.io/public/caddy/stable/gpg.key' | sudo gpg --dearmor -o /usr/share/keyrings/caddy-stable-archive-keyring.gpg &&
            curl -1sLf 'https://dl.cloudsmith.io/public/caddy/stable/debian.deb.txt' | sudo tee /etc/apt/sources.list.d/caddy-stable.list &&
            sudo apt-get update -y &&
            sudo apt-get install -y caddy
        "#;
        cmd!(sh, "bash -c {bash}").run()?;

        // Install Git-core PPA for latest git versions
        // Ref: https://launchpad.net/~git-core/+archive/ubuntu/ppa
        cmd!(sh, "sudo add-apt-repository -y ppa:git-core/ppa").run()?;
        cmd!(sh, "sudo apt-get update -y").run()?;
        cmd!(sh, "sudo apt-get install -y git").run()?;

        // Install Fish Shell
        // Ref: https://fishshell.com/
        cmd!(sh, "sudo add-apt-repository -y ppa:fish-shell/release-3").run()?;
        cmd!(sh, "sudo apt-get update -y").run()?;
        cmd!(sh, "sudo apt-get install -y fish").run()?;

        // Install Vim 9.
        // Ref: https://launchpad.net/~jonathonf/+archive/ubuntu/vim
        cmd!(sh, "sudo add-apt-repository -y ppa:jonathonf/vim").run()?;
        cmd!(sh, "sudo apt-get update -y").run()?;
        cmd!(sh, "sudo apt-get install -y vim-nox").run()?;

        // Install NeoVim.
        // Ref: https://launchpad.net/~jonathonf/+archive/ubuntu/vim
        cmd!(sh, "sudo add-apt-repository -y ppa:neovim-ppa/stable").run()?;
        cmd!(sh, "sudo apt-get -y update").run()?;
        cmd!(sh, "sudo apt-get -y install neovim").run()?;

        // Install NVTOP
        // Ref: https://github.com/Syllo/nvtop#ubuntu--debian
        {
            cmd!(
                sh,
                "sudo apt install cmake libncurses5-dev libncursesw5-dev git"
            )
            .run()?;
            let temp_dir = sh.create_temp_dir()?;
            let _p = sh.push_dir(temp_dir.path());
            cmd!(sh, "git clone https://github.com/Syllo/nvtop.git").run()?;
            let build_dir = sh.create_dir("nvtop/build")?;
            {
                let _p = sh.push_dir(build_dir.as_path());
                cmd!(sh, "cmake .. -DNVIDIA_SUPPORT=ON -DAMDGPU_SUPPORT=ON").run()?;
                cmd!(sh, "make").run()?;
                cmd!(sh, "sudo make install").run()?;
            }
        }

        // Install mosh v1.4.0
        //
        // This is built from source because v1.4.0 is not in 22.04 apt repos.
        // Ref: https://mosh.org/#getting
        {
            cmd!(
                sh,
                "sudo apt install protobuf-compiler libprotobuf-dev libssl-dev libncurses5-dev perl zlib1g-dev"
            )
            .run()?;
            let temp_dir = sh.create_temp_dir()?;
            let _p = sh.push_dir(temp_dir.path());
            cmd!(sh, "curl -LO https://mosh.org/mosh-1.4.0.tar.gz").run()?;
            cmd!(sh, "tar xvf mosh-1.4.0.tar.gz").run()?;
            {
                let _p = sh.push_dir("mosh-1.4.0");
                cmd!(sh, "./configure").run()?;
                cmd!(sh, "make").run()?;
                cmd!(sh, "sudo make install").run()?;
            }
        }
    }

    {
        let _s = Section::new("Installing core packages");
        cmd!(sh, "sudo apt-get install -y {APT_PACKAGES...}").run()?;
    }

    {
        let _s = Section::new("Installing special binaries");

        // Geckodriver for Fonduer.
        //
        // Grab the latext 64-bit linux version.
        let re: &Regex = regex!(r#".*browser_download_url.*"(?P<url>.*linux64\.tar\.gz)"#);
        if let Some(latest_url) = cmd!(
            sh,
            "curl -s https://api.github.com/repos/mozilla/geckodriver/releases/latest"
        )
        .read()?
        .lines()
        .find_map(|line| {
            re.captures(line)
                .map(|caps| caps.name("url").map(|m| m.as_str()).unwrap())
        }) {
            let bash = format!("wget -qO- {} | sudo tar xz -C /usr/local/bin", latest_url);
            cmd!(sh, "bash -c {bash}").run()?;
            cmd!(sh, "sudo chmod +x /usr/local/bin/geckodriver").run()?;
        } else {
            warn!("Skipped installing geckodriver, unable to find latest download.");
        }
    }

    {
        let _s = Section::new("Applying configurations");
    }

    eprintln!("Setup complete. You should reboot this machine :)");

    Ok(())
}
