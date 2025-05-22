use goup_misc::Toolchain;
use goup_misc::ToolchainFilter;
use goup_misc::Version;
use goup_misc::op;

pub(super) fn run(toolchain: String, host: &str) -> anyhow::Result<()> {
    let version = match toolchain.parse()? {
        Toolchain::Stable => op::get_upstream_latest_go_version(host)?,
        Toolchain::Unstable => {
            let version =
                op::list_upstream_go_versions_filter(host, Some(ToolchainFilter::Unstable))?;
            let version = version
                .last()
                .ok_or_else(|| anyhow::anyhow!("failed get latest unstable version"))?;
            version.to_string()
        }
        Toolchain::Beta => {
            let version = op::list_upstream_go_versions_filter(host, Some(ToolchainFilter::Beta))?;
            let version = version
                .last()
                .ok_or_else(|| anyhow::anyhow!("failed get latest beta version"))?;
            version.to_string()
        }
        Toolchain::Version(ver_req) => op::match_version_req(host, &ver_req)?,
        Toolchain::Nightly => {
            anyhow::bail!("gotip is no supported");
        }
    };

    let version = Version::normalize(&version);
    goup_downloader::install_go_version(&version)
}
