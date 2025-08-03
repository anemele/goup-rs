use std::env;

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
    format!("{version}.{os}-{arch}.{ext}")
}

/// archive_sha256 returns `{archive}.sha256`
/// go1.21.5.linux-amd64.tar.gz.sha256, go1.21.5.windows-amd64.zip.sha256
#[inline]
pub fn archive_sha256(archive_filename: &str) -> String {
    format!("{archive_filename}.sha256")
}

/// archive_url returns returns the zip or tar.gz URL of the given Go version.
#[inline]
pub fn archive_url(archive_filename: &str) -> (String, String) {
    let host = goup_misc::consts::go_download_base_url();
    let url0 = format!("{host}/{archive_filename}");
    let url1 = format!("{}.sha256", &url0);
    (url0, url1)
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
    assert!(archive_sha256_url.starts_with(&format!("https://dl.google.com/go/{}", TEST_VERSION)));
}
