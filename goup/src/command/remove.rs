use dialoguer::{MultiSelect, theme::ColorfulTheme};

use goup_misc::op;

pub(super) fn run(version: Vec<String>) -> anyhow::Result<()> {
    if !version.is_empty() {
        let vers = version.iter().map(AsRef::as_ref).collect::<Vec<&str>>();
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
