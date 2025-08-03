use std::collections::HashMap;

use colored::Colorize;

use goup_misc::op;

pub(super) fn run(filter: Option<String>, host: String) -> anyhow::Result<()> {
    let filter = filter.and_then(|s| s.parse().ok());
    let remote_versions = op::list_upstream_go_versions_filter(&host, filter)?;

    let local_versions = op::list_go_version()?;
    let mut v_a_map = HashMap::<String, bool>::new();
    for v in local_versions {
        v_a_map.insert(v.version, v.active);
    }

    #[cfg(windows)]
    colored::control::set_virtual_terminal(true).unwrap();

    for v in remote_versions {
        if !v_a_map.contains_key(&v) {
            println!("  {v}");
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
