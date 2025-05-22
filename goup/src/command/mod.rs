mod cache;
mod env;
mod install;
mod list;
mod remove;
mod search;
mod set;

use clap::Parser;
use goup_misc::consts;
use shadow_rs::shadow;
use std::env::consts::{ARCH, OS};

shadow!(build);
const VERSION: &str = shadow_rs::formatcp!(
    r#"{}
-------------------------------------
{}

Author:          {}
Email:           {}
Repository:      {}
Branch:          {}
GitCommit:       {}
GitFullCommit:   {}
BuildTime:       {}
BuildEnv:        {}, {}
BuildOs:         {}
BuildArch:       {}"#,
    env!("CARGO_PKG_VERSION"),
    env!("CARGO_PKG_DESCRIPTION"),
    build::COMMIT_AUTHOR,
    build::COMMIT_EMAIL,
    env!("CARGO_PKG_REPOSITORY"),
    build::BRANCH,
    build::SHORT_COMMIT,
    build::COMMIT_HASH,
    build::BUILD_TIME_2822,
    build::RUST_VERSION,
    build::RUST_CHANNEL,
    OS,
    ARCH,
);

#[derive(Parser, Debug, PartialEq)]
#[command(author, about, long_about = None)]
#[command(propagate_version = true)]
#[command(version = VERSION)]
#[command(name = "goup")]
#[non_exhaustive] // 表明未来还有其它元素添加
enum Cli {
    /// Install Go with a version
    #[command(visible_aliases = ["i", "add"])]
    Install {
        /// toolchain name, such as 'stable', 'nightly'('tip', 'gotip'), 'unstable', 'beta' or '=1.21.4'
        #[arg(default_value = "stable")]
        toolchain: String,
        /// host that is used to download Go.
        #[arg(long, default_value_t = consts::GO_HOST.to_owned(), env = consts::GOUP_GO_HOST)]
        host: String,
    },

    /// List all installed Go
    #[command(visible_aliases = ["ls", "show"])]
    List,

    /// Remove the specified Go version list.
    /// If no version is provided, a prompt will show to select multiple installed Go version.
    #[command(visible_alias = "rm")]
    Remove {
        /// target go version list.
        version: Vec<String>,
    },

    /// Search Go versions to install
    Search {
        /// a filter, such as 'stable', "unstable", 'beta' or any regex string(1.22.*).
        filter: Option<String>,
        /// host that is used to download Go.
        #[arg(long, default_value_t = consts::GO_HOST.to_owned(), env = consts::GOUP_GO_HOST)]
        host: String,
    },

    /// Set the default Go version to one specified.
    /// If no version is provided, a prompt will show to select a installed Go version.
    #[command(visible_alias = "use")]
    Set {
        /// the version to set.
        version: Option<String>,
    },

    /// Show the specified goup environment variables and values.
    Env,

    /// Clean download archive file
    Clean {
        /// Skip interact prompt.
        #[arg(short, long, default_value_t = false)]
        yes: bool,
    },
}

pub fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();
    use Cli::*;
    match cli {
        Install { toolchain, host } => install::run(toolchain, &host),
        List => list::run(),
        Remove { version } => remove::run(version),
        Search { filter, host } => search::run(filter, host),
        Set { version } => set::run(version),
        Env => env::run(),
        Clean { yes } => cache::run(yes),
    }
}
