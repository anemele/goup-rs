use dialoguer::Confirm;
use dialoguer::theme::ColorfulTheme;
use goup_misc::op;

pub(super) fn run(yes: bool) -> anyhow::Result<()> {
    let confirmation = yes
        || Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Do you want to clean cache file?")
            .interact()?;
    if confirmation {
        op::remove_cache()?;
    } else {
        println!("Cancelled");
    }
    Ok(())
}
