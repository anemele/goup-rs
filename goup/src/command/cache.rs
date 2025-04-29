use clap::Args;
use clap::Subcommand;
use dialoguer::Confirm;
use dialoguer::theme::ColorfulTheme;
use goup_version::Version;

use super::Run;

#[derive(Args, Debug, PartialEq)]
pub struct Cache {
    /// the download command.
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Clone, Debug, PartialEq)]
enum Command {
    /// Show download archive file
    Show {
        /// Contain archive sha256 file
        #[arg(short, long, default_value_t = false)]
        contain_sha256: bool,
    },

    /// Clean download archive file
    Clean {
        /// Skip interact prompt.
        #[arg(short, long, default_value_t = false)]
        yes: bool,
    },
}

impl Run for Cache {
    fn run(&self) -> anyhow::Result<()> {
        match self.command {
            Command::Show { contain_sha256 } => {
                for v in Version::list_cache(contain_sha256)? {
                    println!("{}", v);
                }
            }
            Command::Clean { yes } => {
                let confirmation = yes
                    || Confirm::with_theme(&ColorfulTheme::default())
                        .with_prompt("Do you want to clean cache file?")
                        .interact()?;
                if confirmation {
                    Version::remove_cache()?;
                } else {
                    println!("Cancelled");
                }
            }
        }

        Ok(())
    }
}
