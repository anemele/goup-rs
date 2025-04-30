use anyhow::anyhow;
use clap::Args;

use goup_version::Toolchain;
use goup_version::ToolchainFilter;
use goup_version::Version;
use goup_version::consts;
use goup_version::op;

use super::Run;

#[derive(Args, Debug, PartialEq)]
#[command(disable_version_flag = true)]
pub struct Install {
    /// toolchain name, such as 'stable', 'nightly'('tip', 'gotip'), 'unstable', 'beta' or '=1.21.4'
    #[arg(default_value = "stable")]
    toolchain: String,
    /// an optional change list (CL), If the version is 'tip'
    cl: Option<String>,
    /// host that is used to download Go.
    #[arg(long, default_value_t = consts::GO_HOST.to_owned(), env = consts::GOUP_GO_HOST)]
    host: String,
}

impl Run for Install {
    fn run(&self) -> anyhow::Result<()> {
        let version = match self.toolchain.parse()? {
            Toolchain::Stable => op::get_upstream_latest_go_version(&self.host)?,
            Toolchain::Unstable => {
                let version = op::list_upstream_go_versions_filter(
                    &self.host,
                    Some(ToolchainFilter::Unstable),
                )?;
                let version = version
                    .last()
                    .ok_or_else(|| anyhow!("failed get latest unstable version"))?;
                version.to_string()
            }
            Toolchain::Beta => {
                let version =
                    op::list_upstream_go_versions_filter(&self.host, Some(ToolchainFilter::Beta))?;
                let version = version
                    .last()
                    .ok_or_else(|| anyhow!("failed get latest beta version"))?;
                version.to_string()
            }
            Toolchain::Version(ver_req) => op::match_version_req(&self.host, &ver_req)?,
            Toolchain::Nightly => {
                anyhow::bail!("gotip is no supported");
            }
        };

        let version = Version::normalize(&version);
        goup_downloader::install_go_version(&version)
    }
}
