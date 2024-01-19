use anyhow::anyhow;
use clap::Args;

use goup_downloader::Downloader;
use goup_version::consts;
use goup_version::Toolchain;
use goup_version::ToolchainFilter;
use goup_version::Version;

use super::Run;

#[derive(Args, Debug, PartialEq)]
#[command(disable_version_flag = true)]
pub struct Install {
    /// toolchain name, such as 'stable', 'nightly'('tip', 'gotip'), 'unstable', 'beta' or '1.21.4'('go1.21.4')
    #[arg(default_value = "stable")]
    toolchain: String,
    /// an optional change list (CL), If the version is 'tip'
    cl: Option<String>,
    /// host that is used to download Go.
    #[arg(long, default_value_t = consts::GO_HOST.to_owned(), env = consts::GOUP_GO_HOST)]
    host: String,
    /// only install the version, but do not switch.
    #[arg(long, default_value_t = false)]
    dry: bool,
}

impl Run for Install {
    fn run(&self) -> Result<(), anyhow::Error> {
        let toolchain = self.toolchain.parse()?;
        let version = match toolchain {
            Toolchain::Stable => {
                let version = Version::get_upstream_latest_go_version(&self.host)?;
                let version = Version::normalize(&version);
                println!("Installing {} ...", version);
                Downloader::install_go_version(&version)?;
                version
            }
            Toolchain::Unstable => {
                let version = Version::list_upstream_go_versions(Some(ToolchainFilter::Unstable))?;
                let version = version
                    .last()
                    .ok_or_else(|| anyhow!("failed get latest unstable version"))?;
                let version = Version::normalize(version);
                println!("Installing {} ...", version);
                Downloader::install_go_version(&version)?;
                version
            }
            Toolchain::Beta => {
                let version = Version::list_upstream_go_versions(Some(ToolchainFilter::Beta))?;
                let version = version
                    .last()
                    .ok_or_else(|| anyhow!("failed get latest beta version"))?;
                let version = Version::normalize(version);
                println!("Installing {} ...", version);
                Downloader::install_go_version(&version)?;
                version
            }
            Toolchain::Version(version) => {
                println!("Installing {} ...", version);
                Downloader::install_go_version(&version)?;
                version
            }
            Toolchain::Nightly => {
                println!("Installing gotip ...");
                Downloader::install_go_tip(self.cl.as_deref())?;
                "gotip".to_owned()
            }
        };
        if !self.dry {
            Version::set_go_version(&version)?;
        }
        Ok(())
    }
}
