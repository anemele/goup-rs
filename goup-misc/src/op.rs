use std::fs;
use std::fs::DirEntry;
use std::ops::Deref;
use std::time;

use anyhow::Result;
use anyhow::anyhow;
use indicatif::ProgressBar;
use regex::Regex;
use reqwest::blocking::Client;
use semver::Op;
use semver::VersionReq;
use serde::{Deserialize, Serialize};

use crate::Dir;
use crate::ToolchainFilter;
use crate::Version;

#[derive(Serialize, Deserialize, Debug)]
pub struct GoFile {
    pub arch: String,
    pub filename: String,
    pub kind: String,
    pub os: String,
    pub sha256: String,
    pub size: isize,
    pub version: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct GoRelease {
    pub version: String,
    pub stable: bool,
    // pub files: Vec<GoFile>,
}

pub fn list_upstream_go_versions_filter(
    host: &str,
    filter: Option<ToolchainFilter>,
) -> anyhow::Result<Vec<String>> {
    let ver = list_upstream_go_versions(host)?;
    let re = filter.map_or_else(
        || "(.+)".to_owned(),
        |f| match f {
            ToolchainFilter::Stable => {
                r#"(?:0|[1-9]\d*)\.(?:0|[1-9]\d*)(?:\.(?:0|[1-9]\d*))?\b"#.to_string()
            }
            ToolchainFilter::Unstable => {
                r#"(?:0|[1-9]\d*)\.(?:0|[1-9]\d*)(?:\.(?:0|[1-9]\d*))?(?:rc(?:0|[1-9]\d*))"#
                    .to_string()
            }
            ToolchainFilter::Beta => {
                r#"(?:0|[1-9]\d*)\.(?:0|[1-9]\d*)(?:\.(?:0|[1-9]\d*))?(?:beta(?:0|[1-9]\d*))"#
                    .to_string()
            }
            ToolchainFilter::Filter(s) => format!("(.*{s}.*)"),
        },
    );
    let re = Regex::new(&re)?;
    Ok(ver
        .into_iter()
        .filter_map(|v| re.is_match(&v).then_some(v))
        .collect())
}

/// list upstream go versions from http.
pub fn list_upstream_go_versions(host: &str) -> anyhow::Result<Vec<String>> {
    let spinner = ProgressBar::new_spinner();
    spinner.enable_steady_tick(time::Duration::from_millis(100));

    spinner.set_message("Fetching upstream Go versions");
    let v = Client::builder()
        .timeout(time::Duration::from_secs(10))
        .build()?
        .get(format!("{host}/dl/?mode=json&include=all"))
        .send()?
        .json::<Vec<GoRelease>>()?
        .into_iter()
        .map(|v| v.version.trim_start_matches("go").to_string())
        .rev()
        .collect();
    spinner.finish_and_clear();
    Ok(v)
}

pub fn match_version_req(host: &str, ver_pattern: &str) -> anyhow::Result<String> {
    log::debug!("version request pattern: {ver_pattern}");
    let ver_req = VersionReq::parse(ver_pattern)?;
    // 是否是精确匹配, 如果是则直接返回
    if ver_req.comparators.iter().all(|v| v.op == Op::Exact) {
        return Ok(ver_pattern.trim_start_matches('=').to_owned());
    }
    for ver in list_upstream_go_versions(host)?.iter().rev() {
        if ver_req.matches(&Version::semantic(ver)?) {
            return Ok(ver.to_owned());
        }
    }
    Err(anyhow!("not any match version!"))
}

/// get upstream latest go version.
pub fn get_upstream_latest_go_version(host: &str) -> anyhow::Result<String> {
    let spinner = ProgressBar::new_spinner();
    spinner.enable_steady_tick(time::Duration::from_millis(100));

    spinner.set_message("Fetching upstream latest Go version");
    let body = Client::builder()
        .timeout(time::Duration::from_secs(10))
        .build()?
        .get(format!("{host}/VERSION?m=text"))
        .send()?
        .text()?;
    let v = body
        .split('\n')
        .next()
        .ok_or_else(|| anyhow!("Getting latest Go version failed"))
        .map(|v| v.to_owned());
    spinner.finish_and_clear();
    v
}

/// list locally installed go version.
pub fn list_go_version() -> anyhow::Result<Vec<Version>> {
    let goup_home = Dir::goup_home()?;
    // may be .goup not exist
    if !goup_home.exists() {
        return Ok(Vec::new());
    }

    // may be current not exist
    let current = goup_home.current().read_link();
    let current = current.as_ref();
    let dir: Result<Vec<DirEntry>, _> = goup_home.read_dir()?.collect();
    let mut version_dirs: Vec<_> = dir?
        .iter()
        .filter_map(|v| {
            if !v.path().is_dir() {
                return None;
            }

            let ver = v.file_name().to_string_lossy().to_string();
            if ver != "gotip" && !goup_home.is_dot_unpacked_success_file_exists(&ver) {
                return None;
            }
            Some(Version {
                version: ver.trim_start_matches("go").into(),
                active: current.is_ok_and(|vv| vv == goup_home.version_go(ver).deref()),
            })
        })
        .collect();
    version_dirs.sort();
    Ok(version_dirs)
}

/// set active go version
pub fn set_go_version(version: &str) -> anyhow::Result<()> {
    let version = Version::normalize(version);
    let goup_home = Dir::goup_home()?;
    let original = goup_home.version_go(&version);
    if !original.exists() {
        anyhow::bail!("Go version {version} is not installed. Install it with `goup install`.");
    }

    let link = goup_home.current();
    let _ = fs::remove_dir_all(&link);
    #[cfg(unix)]
    {
        use std::os::unix::fs as unix_fs;
        unix_fs::symlink(original, &link)?;
    }
    #[cfg(windows)]
    {
        junction::create(original, &link)?;
    }

    println!("Default Go is set to '{version}'");
    Ok(())
}

/// remove the go version, if it is current active go version, will ignore deletion.
pub fn remove_go_version(version: &str) -> anyhow::Result<()> {
    let version = Version::normalize(version);
    let version_dir = Dir::goup_home()?.version(&version);
    if version_dir.exists() {
        fs::remove_dir_all(&version_dir)?;
    }

    if let Some(cur) = current_go_version()? {
        if cur == version {
            log::warn!("{version} is the active version.");
        }
    }

    Ok(())
}

/// remove multiple go version, if it is current active go version, will ignore deletion.
pub fn remove_go_versions(vers: &[&str]) -> anyhow::Result<()> {
    let spinner = ProgressBar::new_spinner();
    spinner.enable_steady_tick(time::Duration::from_millis(100));

    for (i, ver) in vers.iter().enumerate() {
        spinner.set_message(format!("Removing {ver} ({}/{})", i + 1, vers.len()));
        remove_go_version(ver)?;
    }
    spinner.finish_with_message(format!("Removed {}", vers.len()));

    Ok(())
}

/// current active go version
pub fn current_go_version() -> anyhow::Result<Option<String>> {
    // may be current not exist
    let current = Dir::goup_home()?.current().read_link().ok().and_then(|p| {
        p.parent()
            .and_then(|v| v.file_name().map(|vv| vv.to_string_lossy().to_string()))
    });
    Ok(current)
}

/// list `${HOME}/.goup/cache` directory items(only file, ignore directory).
pub fn list_cache(contain_sha256: bool) -> anyhow::Result<Vec<String>> {
    let goup_home = Dir::goup_home()?;
    // may be .goup or .goup/cache not exist
    if !goup_home.exists() || !goup_home.cache().exists() {
        return Ok(Vec::new());
    }
    let dir: Result<Vec<DirEntry>, _> = goup_home.cache().read_dir()?.collect();
    let mut archive_files: Vec<_> = dir?
        .iter()
        .filter_map(|v| {
            if v.path().is_dir() {
                return None;
            }
            let filename = v.file_name();
            let filename = filename.to_string_lossy();
            (contain_sha256 || !filename.ends_with(".sha256")).then(|| filename.to_string())
        })
        .collect();
    archive_files.sort();
    Ok(archive_files)
}

/// remove `${HOME}/.goup/cache` directory.
pub fn remove_cache() -> anyhow::Result<()> {
    let dl_dir = Dir::goup_home()?.cache();
    if dl_dir.exists() {
        fs::remove_dir_all(&dl_dir)?;
    }
    Ok(())
}

/// remove `${HOME}/.goup` directory.
pub fn remove_goup_home() -> anyhow::Result<()> {
    let goup_home_dir = Dir::goup_home()?;
    if goup_home_dir.exists() {
        fs::remove_dir_all(&goup_home_dir)?;
    }
    Ok(())
}
