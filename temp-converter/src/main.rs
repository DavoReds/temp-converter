mod cli;
mod float;

use clap::Parser;
use cli::Cli;
use float::Float;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let value = Float::from_str(&cli.value, cli.double)?;
    let result = value.convert(cli.from, cli.to);

    println!("{result}");

    Ok(())
}
