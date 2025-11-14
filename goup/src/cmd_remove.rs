use dialoguer::{MultiSelect, theme::ColorfulTheme};

use goup_misc::op;

pub(super) fn run(version: Vec<String>) -> anyhow::Result<()> {
    if !version.is_empty() {
        return op::remove_go_versions(&version);
    }

    let vers = op::list_go_version()?;
    if vers.is_empty() {
        anyhow::bail!("No go is installed");
    }
    let items: Vec<String> = vers.iter().map(|v| v.version.to_string()).collect();
    let selection = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select multiple version")
        .items(&items)
        .interact()?;
    if selection.is_empty() {
        anyhow::bail!("No item selected");
    }

    let vers: Vec<String> = selection.into_iter().map(|i| items[i].clone()).collect();
    op::remove_go_versions(&vers)
}
