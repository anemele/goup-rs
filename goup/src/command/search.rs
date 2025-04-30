use std::collections::HashMap;

use clap::Args;
use colored::Colorize;

use super::Run;
use goup_misc::consts;
use goup_misc::op;

#[derive(Args, Debug, PartialEq)]
pub struct Search {
    /// a filter, such as 'stable', "unstable", 'beta' or any regex string(1.22.*).
    filter: Option<String>,
    /// host that is used to download Go.
    #[arg(long, default_value_t = consts::GO_HOST.to_owned(), env = consts::GOUP_GO_HOST)]
    host: String,
}

impl Run for Search {
    fn run(&self) -> anyhow::Result<()> {
        let filter = self.filter.as_ref().and_then(|s| s.parse().ok());
        let remote_versions = op::list_upstream_go_versions_filter(&self.host, filter)?;

        let local_versions = op::list_go_version()?;
        let mut v_a_map = HashMap::<String, bool>::new();
        for v in local_versions {
            v_a_map.insert(v.version, v.active);
        }

        #[cfg(windows)]
        colored::control::set_virtual_terminal(true).unwrap();

        for v in remote_versions {
            if !v_a_map.contains_key(&v) {
                println!("  {}", v);
                continue;
            }
            if v_a_map[&v] {
                println!("* {}", v.green());
            } else {
                println!("  {}", v.green());
            }
        }

        Ok(())
    }
}
