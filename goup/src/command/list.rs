use clap::Args;
use colored::{Colorize, control::set_virtual_terminal};
use which::which;

use super::Run;
use goup_version::Version;

#[derive(Args, Debug, PartialEq)]
pub struct List;

impl Run for List {
    fn run(&self) -> Result<(), anyhow::Error> {
        let vers = Version::list_go_version()?;
        if vers.is_empty() {
            println!("No Go is installed by goup.");
            if let Ok(go_bin) = which("go") {
                println!(" Using system Go {}.", go_bin.to_string_lossy());
            }
        } else {
            #[cfg(target_family = "windows")]
            set_virtual_terminal(true).unwrap();

            for v in vers {
                if v.active {
                    println!("{}", format!("* {}", v.version).green());
                } else {
                    println!("  {}", v.version);
                };
            }
        }

        Ok(())
    }
}
