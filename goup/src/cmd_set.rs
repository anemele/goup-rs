use dialoguer::{Select, theme::ColorfulTheme};

use goup_misc::op;

pub(super) fn run(version: Option<String>) -> anyhow::Result<()> {
    if let Some(version) = version {
        return op::set_go_version(&version);
    }

    let vers = op::list_go_version()?;
    if vers.is_empty() {
        anyhow::bail!("Not any go is installed, Install it with `goup install`.");
    }

    let mut items = vec![];
    let mut pos = 0;
    for (i, v) in vers.iter().enumerate() {
        items.push(v.version.to_string());
        if v.active {
            pos = i;
        }
    }
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a version")
        .items(&items)
        .default(pos)
        .interact()?;
    op::set_go_version(&items[selection])
}
