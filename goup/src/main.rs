use clap::Parser;

use goup_rs::Cli;
use goup_rs::Run;

fn main() -> anyhow::Result<()> {
    Cli::parse().run()
}
