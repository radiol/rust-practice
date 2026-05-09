use anyhow::Result;
use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};

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

fn main() {
    if let Err(e) = run(Cli::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> Result<()> {
    let num_files = cli.files.len();
    for (i, filename) in cli.files.iter().enumerate() {
        match open(filename) {
            Err(err) => {
                eprintln!("{filename}: {err}");
            }
            Ok(mut file) => {
                if num_files > 1 {
                    println!("{}==> {filename} <==", if i > 0 { "\n" } else { "" });
                }
                if let Some(num_bytes) = cli.bytes {
                    let mut buf = vec![0; num_bytes as usize];
                    let bytes_read = file.read(&mut buf)?;
                    print!("{}", String::from_utf8_lossy(&buf[..bytes_read]));
                } else {
                    let mut line = String::new();
                    for _ in 0..cli.lines {
                        let bytes = file.read_line(&mut line)?;
                        if bytes == 0 {
                            break;
                        }
                        print!("{}", line);
                        line.clear();
                    }
                }
            }
        }
    }
    Ok(())
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(std::io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
