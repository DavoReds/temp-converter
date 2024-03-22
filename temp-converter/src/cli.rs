use clap::{Parser, ValueEnum};
use clap_stdin::MaybeStdin;

#[derive(Parser, Debug)]
#[command(
    name = "Temperature Converter",
    author,
    version,
    about,
    long_about = None,
    help_template = "\
{before-help}{name} {version}
{author-with-newline}{about-with-newline}
{usage-heading} {usage}

{all-args}{after-help}"
)]
pub struct Cli {
    /// Unit to convert from
    #[arg(value_enum)]
    pub from: Unit,

    /// Unit to convert to
    #[arg(value_enum)]
    pub to: Unit,

    /// Use double precision
    #[arg(short, long)]
    pub double: bool,

    /// Value to convert. `-` for stdin
    pub value: MaybeStdin<String>,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum Unit {
    #[value(alias = "c")]
    Celsius,

    #[value(alias = "f")]
    Fahrenheit,

    #[value(alias = "k")]
    Kelvin,
}
