use goup_misc::{Dir, consts};

#[inline]
fn print_env(key: &str, value: &str) {
    #[cfg(windows)]
    println!("set {}={}", key, value);
    #[cfg(unix)]
    println!("{}={}", key, value);
}

pub(super) fn run() -> anyhow::Result<()> {
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
