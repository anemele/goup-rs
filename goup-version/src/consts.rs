use std::env;

pub const GOUP_HOME: &str = "GOUP_HOME";
pub const GOUP_GO_HOST: &str = "GOUP_GO_HOST";
pub const GOUP_GO_DOWNLOAD_BASE_URL: &str = "GOUP_GO_DOWNLOAD_BASE_URL";

pub const GO_HOST: &str = "https://golang.google.cn"; // "https://go.dev";
pub const GO_DOWNLOAD_BASE_URL: &str = "https://golang.google.cn/dl"; //"https://dl.google.com/go";

#[inline]
fn get_var_or_else(key: &str, val: &str) -> String {
    if let Ok(s) = env::var(key) {
        if !s.is_empty() {
            return s;
        }
    }
    val.to_string()
}

pub fn go_host() -> String {
    get_var_or_else(GOUP_GO_HOST, GO_HOST)
}

pub fn go_download_base_url() -> String {
    get_var_or_else(GOUP_GO_DOWNLOAD_BASE_URL, GO_DOWNLOAD_BASE_URL)
}

/// go_version_archive returns the zip or tar.gz of the given Go version.
/// go1.21.5.linux-amd64.tar.gz, go1.21.5.windows-amd64.zip
pub fn go_version_archive(version: &str) -> String {
    let os = match env::consts::OS {
        "macos" => "darwin",
        os => os,
    };
    let arch = match (os, env::consts::ARCH) {
        (_, "x86") => "386",
        (_, "x86_64") => "amd64",
        ("linux", "arm") => "armv6l",
        (_, "aarch64") => "arm64",
        _ => env::consts::ARCH,
    };
    let ext = if os == "windows" { "zip" } else { "tar.gz" };
    format!("{}.{}-{}.{}", version, os, arch, ext)
}

/// archive_sha256 returns `{archive}.sha256`
/// go1.21.5.linux-amd64.tar.gz.sha256, go1.21.5.windows-amd64.zip.sha256
#[inline]
pub fn archive_sha256(archive_filename: &str) -> String {
    format!("{}.sha256", archive_filename)
}

/// archive_url returns returns the zip or tar.gz URL of the given Go version.
#[inline]
pub fn archive_url(archive_filename: &str) -> (String, String) {
    let host = go_download_base_url();
    let url0 = format!("{}/{}", host, archive_filename);
    let url1 = format!("{}.sha256", &url0);
    (url0, url1)
}

#[cfg(test)]
mod tests {
    use crate::consts::{archive_sha256, archive_url, go_version_archive};

    use super::{GO_DOWNLOAD_BASE_URL, GO_HOST};
    use super::{GOUP_GO_DOWNLOAD_BASE_URL, GOUP_GO_HOST};
    use super::{go_download_base_url, go_host};

    #[test]
    fn test_env_vars_unset() {
        temp_env::with_vars_unset([GOUP_GO_HOST, GOUP_GO_DOWNLOAD_BASE_URL], || {
            assert_eq!(go_host(), GO_HOST);
            assert_eq!(go_download_base_url(), GO_DOWNLOAD_BASE_URL);
        })
    }

    #[test]
    fn test_env_vars_set() {
        let test_go_host = "https://golang.google.cn";
        let test_go_download_base_url = "https://golang.google.cn/dl";
        temp_env::with_vars(
            [
                (GOUP_GO_HOST, Some(test_go_host)),
                (GOUP_GO_DOWNLOAD_BASE_URL, Some(test_go_download_base_url)),
            ],
            || {
                assert_eq!(go_host(), test_go_host);
                assert_eq!(go_download_base_url(), test_go_download_base_url);
            },
        )
    }

    #[test]
    fn test_archive() {
        const TEST_VERSION: &str = "1.21.5";
        let archive_filename = go_version_archive(TEST_VERSION);
        #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
        assert_eq!(
            archive_filename,
            format!("{}.darwin-amd64.tar.gz", TEST_VERSION)
        );
        #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
        assert_eq!(
            archive_filename,
            format!("{}.darwin-arm64.tar.gz", TEST_VERSION)
        );

        #[cfg(all(target_os = "linux", target_arch = "x86"))]
        assert_eq!(
            archive_filename,
            format!("{}.linux-386.tar.gz", TEST_VERSION)
        );
        #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
        assert_eq!(
            archive_filename,
            format!("{}.linux-amd64.tar.gz", TEST_VERSION)
        );
        #[cfg(all(target_os = "linux", target_arch = "aarch64"))]
        assert_eq!(
            archive_filename,
            format!("{}.linux-arm64.tar.gz", TEST_VERSION)
        );
        #[cfg(all(target_os = "linux", target_arch = "arm"))]
        assert_eq!(
            archive_filename,
            format!("{}.linux-armv6l.tar.gz", TEST_VERSION)
        );
        #[cfg(all(target_os = "windows", target_arch = "x86"))]
        assert_eq!(
            archive_filename,
            format!("{}.windows-386.zip", TEST_VERSION)
        );
        #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
        assert_eq!(
            archive_filename,
            format!("{}.windows-amd64.zip", TEST_VERSION)
        );

        assert!(archive_sha256(&archive_filename).ends_with(".sha256"));

        let (archive_url, archive_sha256_url) = archive_url(&archive_filename);
        assert!(archive_url.starts_with(&format!("https://dl.google.com/go/{}", TEST_VERSION)));
        assert!(
            archive_sha256_url.starts_with(&format!("https://dl.google.com/go/{}", TEST_VERSION))
        );
    }
}
