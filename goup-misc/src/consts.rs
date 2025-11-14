use std::env;

pub const GOUP_HOME: &str = "GOUP_HOME";
pub const GOUP_GO_HOST: &str = "GOUP_GO_HOST";
pub const GOUP_GO_DOWNLOAD_BASE_URL: &str = "GOUP_GO_DOWNLOAD_BASE_URL";

pub const GO_HOST: &str = "https://golang.google.cn"; // "https://go.dev"; //
pub const GO_DOWNLOAD_BASE_URL: &str = "https://dl.google.com/go";

#[inline]
fn get_var_or_else(key: &str, val: &str) -> String {
    if let Ok(s) = env::var(key)
        && !s.is_empty()
    {
        return s;
    }
    val.to_string()
}

pub fn go_host() -> String {
    get_var_or_else(GOUP_GO_HOST, GO_HOST)
}

pub fn go_download_base_url() -> String {
    get_var_or_else(GOUP_GO_DOWNLOAD_BASE_URL, GO_DOWNLOAD_BASE_URL)
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
