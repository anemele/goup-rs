use clap::Args;
use goup_version::{Dir, consts};

use super::Run;

#[derive(Args, Debug, PartialEq)]
pub struct Env;

#[inline]
fn print_env(key: &str, value: &str) {
    #[cfg(windows)]
    println!("set {}={}", key, value);
    #[cfg(unix)]
    println!("{}={}", key, value);
}

impl Run for Env {
    fn run(&self) -> anyhow::Result<()> {
        print_env(
            consts::GOUP_HOME,
            &Dir::goup_home().unwrap_or_default().to_string_lossy(),
        );
        print_env(consts::GOUP_GO_HOST, &consts::go_host());
        print_env(
            consts::GOUP_GO_DOWNLOAD_BASE_URL,
            &consts::go_download_base_url(),
        );

        Ok(())
    }
}
