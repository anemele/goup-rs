use colored::Colorize;
use which::which;

use goup_misc::op;

pub(super) fn run() -> anyhow::Result<()> {
    let vers = op::list_go_version()?;
    if vers.is_empty() {
        println!("No Go is installed by goup.");
        if let Ok(go_bin) = which("go") {
            println!(" Using system Go {}.", go_bin.to_string_lossy());
        }
    } else {
        #[cfg(windows)]
        colored::control::set_virtual_terminal(true).unwrap();

        for v in vers {
            if v.active {
                println!("* {}", v.version.to_string().green());
            } else {
                println!("  {}", v.version);
            };
        }
    }

    Ok(())
}
