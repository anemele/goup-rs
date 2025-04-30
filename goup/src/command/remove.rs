use clap::Args;
use dialoguer::{MultiSelect, theme::ColorfulTheme};

use goup_misc::op;

use super::Run;

#[derive(Args, Debug, PartialEq)]
#[command(disable_version_flag = true)]
pub struct Remove {
    /// target go version list.
    version: Vec<String>,
}

impl Run for Remove {
    fn run(&self) -> anyhow::Result<()> {
        if !self.version.is_empty() {
            let vers = self
                .version
                .iter()
                .map(AsRef::as_ref)
                .collect::<Vec<&str>>();
            return op::remove_go_versions(&vers);
        }

        let vers = op::list_go_version()?;
        if vers.is_empty() {
            anyhow::bail!("No go is installed");
        }
        let items: Vec<&str> = vers.iter().map(|v| v.version.as_ref()).collect();
        let selection = MultiSelect::with_theme(&ColorfulTheme::default())
            .with_prompt("Select multiple version")
            .items(&items)
            .interact()?;
        if selection.is_empty() {
            anyhow::bail!("No item selected");
        }

        let vers = selection
            .into_iter()
            .map(|i| items[i])
            .collect::<Vec<&str>>();
        op::remove_go_versions(&vers)
    }
}
