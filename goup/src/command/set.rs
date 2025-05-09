use clap::Args;
use dialoguer::{Select, theme::ColorfulTheme};

use goup_misc::op;

use super::Run;

#[derive(Args, Debug, PartialEq)]
#[command(disable_version_flag = true)]
pub struct Set {
    /// target go version
    version: Option<String>,
}

impl Run for Set {
    fn run(&self) -> anyhow::Result<()> {
        if let Some(version) = &self.version {
            return op::set_go_version(version);
        }

        let vers = op::list_go_version()?;
        if vers.is_empty() {
            anyhow::bail!("Not any go is installed, Install it with `goup install`.");
        }

        let mut items = Vec::new();
        let mut pos = 0;
        for (i, v) in vers.iter().enumerate() {
            items.push(v.version.as_ref());
            if v.active {
                pos = i;
            }
        }
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select a version")
            .items(&items)
            .default(pos)
            .interact()?;
        op::set_go_version(items[selection])
    }
}
