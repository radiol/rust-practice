use clap::Parser;
use anyhow::Result;

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Cli {
    #[arg(value_name("FILES"), default_value("-"))]
    files: Vec<String>,
    #[arg(
        short('n'),
        long,
        value_name("LINES"),
        default_value("10"),
        conflicts_with("bytes"),
        value_parser(clap::value_parser!(u64).range(1..))
    )]
    lines: u64,
    #[arg(
        short('c'),
        long,
        value_name("BYTES"),
        conflicts_with("lines"),
        value_parser(clap::value_parser!(u64).range(1..))
    )]
    bytes: Option<u64>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    dbg!(&cli);
    Ok(())
}
