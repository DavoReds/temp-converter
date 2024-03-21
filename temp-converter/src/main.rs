use clap::Parser;
use cli::Cli;

mod cli;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();

    Ok(())
}
